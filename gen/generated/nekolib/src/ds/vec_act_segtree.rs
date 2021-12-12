//! `Vec` ベースの区間作用セグ木。

use super::super::traits::act;
use super::super::traits::action;
use super::super::traits::binop;
use super::super::traits::fold;
use super::super::traits::fold_bisect;
use super::super::utils::buf_range;

use std::cell::RefCell;
use std::ops::{Range, RangeBounds};

use act::Act;
use action::MonoidAction;
use binop::{Identity, Magma};
use buf_range::bounds_within;
use fold::Fold;
use fold_bisect::{FoldBisect, FoldBisectRev};

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

    fn nodes(&self, l: usize, r: usize) -> Vec<usize> {
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
        vr.reverse();
        vl.append(&mut vr);
        vl
    }

    fn nodes_rev(&self, l: usize, r: usize) -> Vec<usize> {
        self.nodes(l, r).into_iter().rev().collect()
    }

    fn build(&mut self, mut i: usize) {
        let mut buf = self.buf.borrow_mut();
        let def = self.def.borrow();
        while i > 1 {
            i >>= 1;
            buf[i] = self
                .action
                .operand()
                .op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
            self.action.act(&mut buf[i], def[i].clone());
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

    fn push_down(&self, i: usize) {
        let e = self.action.operator().id();
        let d = std::mem::replace(&mut self.def.borrow_mut()[i], e.clone());
        if d != e {
            self.apply(i << 1, d.clone());
            self.apply(i << 1 | 1, d);
        }
    }

    fn resolve(&self, i: usize) {
        for h in (1..=(i + 1).next_power_of_two().trailing_zeros() - 1).rev() {
            self.push_down(i >> h);
        }
    }

    fn resolve_all(&self) {
        let e = self.action.operator().id();
        for i in 1..self.len {
            let d = std::mem::replace(&mut self.def.borrow_mut()[i], e.clone());
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
        v.resolve_all();
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
        let mut il = self.len + start;
        let mut ir = self.len + end;
        if il >= ir {
            return self.action.operand().id();
        }
        self.resolve(il);
        self.resolve(ir - 1);
        let mut res_l = self.action.operand().id();
        let mut res_r = self.action.operand().id();
        let buf = self.buf.borrow();
        while il < ir {
            if il & 1 == 1 {
                let tmp = buf[il].clone();
                res_l = self.action.operand().op(res_l, tmp);
                il += 1;
            }
            if ir & 1 == 1 {
                ir -= 1;
                let tmp = buf[ir].clone();
                res_r = self.action.operand().op(tmp, res_r);
            }
            il >>= 1;
            ir >>= 1;
        }
        self.action.operand().op(res_l, res_r)
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
        let mut il = self.len + start;
        let mut ir = self.len + end;
        if il >= ir {
            return;
        }
        self.resolve(il);
        self.resolve(ir - 1);
        while il < ir {
            if il & 1 == 1 {
                self.apply(il, op.clone());
                il += 1;
            }
            if ir & 1 == 1 {
                ir -= 1;
                self.apply(ir, op.clone());
            }
            il >>= 1;
            ir >>= 1;
        }
        self.build(self.len + start);
        self.build(self.len + end - 1);
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

        let mut x = self.action.operand().id();
        assert!(pred(&x), "`pred(id)` must hold");
        match self.fold(l..) {
            x if pred(&x) => return (self.len, x),
            _ => {}
        }

        self.resolve(self.len + l);
        self.resolve(self.len + self.len - 1);

        for v in self.nodes(l, self.len) {
            let tmp = self
                .action
                .operand()
                .op(x.clone(), self.buf.borrow()[v].clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                self.push_down(v);
                v <<= 1;
                let tmp = self
                    .action
                    .operand()
                    .op(x.clone(), self.buf.borrow()[v].clone());
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

        let mut x = self.action.operand().id();
        assert!(pred(&x), "`pred(id)` must hold");
        match self.fold(..r) {
            x if pred(&x) => return (0, x),
            _ => {}
        }

        self.resolve(self.len);
        self.resolve(self.len + r - 1);

        for v in self.nodes_rev(0, r) {
            let tmp = self
                .action
                .operand()
                .op(self.buf.borrow()[v].clone(), x.clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                self.push_down(v);
                v = v << 1 | 1;
                let tmp = self
                    .action
                    .operand()
                    .op(self.buf.borrow()[v].clone(), x.clone());
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
