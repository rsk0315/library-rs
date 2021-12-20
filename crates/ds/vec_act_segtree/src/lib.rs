//! `Vec` ベースの区間作用セグ木。

use std::cell::RefCell;
use std::ops::{Range, RangeBounds};

use act::Act;
use action::MonoidAction;
use binop::{Identity, Magma};
use buf_range::bounds_within;
use fold::Fold;
use fold_bisect::{FoldBisect, FoldBisectRev};

const WORD_SIZE: usize = 0_usize.count_zeros() as usize;

fn bsr(i: usize) -> usize { WORD_SIZE - 1 - i.leading_zeros() as usize }

#[derive(Clone)]
pub struct VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    buf: RefCell<Vec<<A::Operand as Magma>::Set>>,
    def: RefCell<Vec<<A::Operator as Magma>::Set>>,
    len: usize,
    action: A,
}

impl<A> VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    #[must_use]
    pub fn new(len: usize) -> Self
    where
        A: Default,
    {
        let action = A::default();
        Self {
            len,
            buf: RefCell::new(vec![action.operand().id(); len + len]),
            def: RefCell::new(vec![action.operator().id(); len]),
            action,
        }
    }

    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn len(&self) -> usize { self.len }

    fn arch_pair(&self, l: usize, r: usize) -> (Vec<usize>, Vec<usize>) {
        let mut l = self.len + l;
        let mut r = self.len + r;
        let mut vl = vec![];
        let mut vr = vec![];
        while l < r {
            if l & 1 == 1 {
                vl.push(l);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr.push(r);
            }
            l >>= 1;
            r >>= 1;
        }
        (vl, vr)
    }

    fn arch(&self, l: usize, r: usize) -> Vec<usize> {
        let (mut vl, vr) = self.arch_pair(l, r);
        vl.extend(vr.into_iter().rev());
        vl
    }

    fn arch_rev(&self, l: usize, r: usize) -> Vec<usize> {
        let (vl, mut vr) = self.arch_pair(l, r);
        vr.extend(vl.into_iter().rev());
        vr
    }

    fn build_range(&mut self, start: usize, end: usize) {
        let mut buf = self.buf.borrow_mut();
        let def = self.def.borrow();
        let action = &self.action;
        let operand = action.operand();
        for i in self.ancestors_upward(start, end) {
            buf[i] = operand.op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
            action.act(&mut buf[i], def[i].clone());
        }
    }

    fn apply(&self, i: usize, op: <A::Operator as Magma>::Set) {
        let mut buf = self.buf.borrow_mut();
        let mut def = self.def.borrow_mut();
        self.action.act(&mut buf[i], op.clone());
        if i < self.len {
            def[i] = self.action.operator().op(def[i].clone(), op);
        }
    }

    fn force(&self, i: usize) {
        let e = self.action.operator().id();
        let d = {
            let mut def = self.def.borrow_mut();
            std::mem::replace(&mut def[i], e.clone())
        };
        if d != e {
            self.apply(i << 1, d.clone());
            self.apply(i << 1 | 1, d);
        }
    }

    fn ancestors_downward(&self, start: usize, end: usize) -> Vec<usize> {
        let mut res = vec![];
        let l = self.len + start;
        res.extend((1..=bsr(l)).rev().map(|i| l >> i));
        if start == end {
            return res;
        }
        let r = self.len + end - 1;
        if l.leading_zeros() == r.leading_zeros() {
            if l != r {
                res.extend((1..=bsr(l ^ r)).rev().map(|i| r >> i));
            }
        } else {
            if l != r >> 1 {
                res.extend((1..=bsr(l ^ (r >> 1))).rev().map(|i| r >> (i + 1)));
                res.push(r >> 1);
            }
        }
        res
    }

    fn ancestors_upward(&self, start: usize, end: usize) -> Vec<usize> {
        let mut res = vec![];
        let mut l = self.len + start;
        if start == end {
            while l > 1 {
                l >>= 1;
                res.push(l);
            }
            return res;
        }

        let mut r = self.len + end - 1;
        if l.leading_zeros() != r.leading_zeros() {
            r >>= 1;
            res.push(r);
        }

        while l != r {
            l >>= 1;
            r >>= 1;
            res.push(l);
            res.push(r);
        }
        while l > 1 {
            l >>= 1;
            res.push(l);
        }
        res
    }

    fn force_range(&self, l: usize, r: usize) {
        for i in self.ancestors_downward(l, r) {
            self.force(i);
        }
    }

    fn force_all(&self) {
        let mut def = self.def.borrow_mut();
        let e = self.action.operator().id();
        for i in 1..self.len {
            let d = std::mem::replace(&mut def[i], e.clone());
            self.apply(i, d);
        }
    }
}

impl<A> From<Vec<<A::Operand as Magma>::Set>> for VecActSegtree<A>
where
    A: MonoidAction + Default,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    fn from(v: Vec<<A::Operand as Magma>::Set>) -> Self {
        Self::from((v, A::default()))
    }
}

impl<A> From<(Vec<<A::Operand as Magma>::Set>, A)> for VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    fn from((mut v, action): (Vec<<A::Operand as Magma>::Set>, A)) -> Self {
        let len = v.len();
        let mut buf = vec![action.operand().id(); len];
        buf.append(&mut v);
        for i in (0..len).rev() {
            buf[i] = action
                .operand()
                .op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
        let buf = RefCell::new(buf);
        let def = RefCell::new(vec![action.operator().id(); len]);
        Self { buf, def, len, action }
    }
}

impl<A> From<VecActSegtree<A>> for Vec<<A::Operand as Magma>::Set>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    fn from(v: VecActSegtree<A>) -> Self {
        v.force_all();
        v.buf.into_inner()
    }
}

impl<A, B> Fold<B> for VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
    B: RangeBounds<usize>,
{
    type Output = A::Operand;
    fn fold(&self, b: B) -> <Self::Output as Magma>::Set {
        let Range { start, end } = bounds_within(b, self.len);
        let operand = self.action.operand();
        if start >= end {
            return operand.id();
        }
        self.force_range(start, end);
        let mut res = operand.id();
        let buf = self.buf.borrow();
        for v in self.arch(start, end) {
            res = operand.op(res, buf[v].clone());
        }
        res
    }
}

impl<A, B> Act<B> for VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
    B: RangeBounds<usize>,
{
    type Action = A;
    fn act(&mut self, b: B, op: <A::Operator as Magma>::Set) {
        let Range { start, end } = bounds_within(b, self.len);
        if start >= end {
            return;
        }
        self.force_range(start, end);
        for v in self.arch(start, end) {
            self.apply(v, op.clone());
        }
        self.build_range(start, end);
    }
}

impl<A> FoldBisect for VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    fn fold_bisect<F>(
        &self,
        l: usize,
        pred: F,
    ) -> (usize, <A::Operand as Magma>::Set)
    where
        F: Fn(&<A::Operand as Magma>::Set) -> bool,
    {
        assert!(
            l < self.len,
            "index out of bound: the len is {} but the index is {}; valid range: 0..{}",
            self.len, l, self.len
        );

        let operand = self.action.operand();
        let mut x = operand.id();
        assert!(pred(&x), "`pred(id)` must hold");
        match self.fold(l..) {
            x if pred(&x) => return (self.len, x),
            _ => {}
        }

        self.force_range(l, self.len);
        let buf = self.buf.borrow();
        for v in self.arch(l, self.len) {
            let tmp = operand.op(x.clone(), buf[v].clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                self.force(v);
                v <<= 1;
                let tmp = operand.op(x.clone(), buf[v].clone());
                if pred(&x) {
                    x = tmp;
                    v += 1;
                }
            }
            return (v - self.len, x);
        }
        unreachable!();
    }
}

impl<A> FoldBisectRev for VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    fn fold_bisect_rev<F>(
        &self,
        r: usize,
        pred: F,
    ) -> (usize, <A::Operand as Magma>::Set)
    where
        F: Fn(&<A::Operand as Magma>::Set) -> bool,
    {
        assert!(
            r <= self.len,
            "index out of bounds: the len is {} but the index is {}; valid range: 0..={}",
            self.len, r, self.len
        );

        let operand = self.action.operand();
        let mut x = operand.id();
        assert!(pred(&x), "`pred(id)` must hold");
        match self.fold(..r) {
            x if pred(&x) => return (0, x),
            _ => {}
        }

        self.force_range(0, r);
        let buf = self.buf.borrow();
        for v in self.arch_rev(0, r) {
            let tmp = operand.op(buf[v].clone(), x.clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                self.force(v);
                v = v << 1 | 1;
                let tmp = operand.op(buf[v].clone(), x.clone());
                if pred(&tmp) {
                    x = tmp;
                    v -= 1;
                }
            }
            return (v - self.len + 1, x);
        }
        unreachable!();
    }
}
