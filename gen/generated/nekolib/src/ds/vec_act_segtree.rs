//! `Vec` ベースの区間作用セグ木。

use super::super::traits::act;
use super::super::traits::action;
use super::super::traits::additive;
use super::super::traits::binop;
use super::super::traits::fold;
use super::super::traits::fold_bisect;
use super::super::traits::min;
use super::super::utils::buf_range;
use super::super::utils::op_add;
use super::super::utils::op_max;

use std::cell::RefCell;
use std::ops::{Range, RangeBounds};

use act::Act;
use action::MonoidAction;
use binop::{Identity, Magma, Monoid};
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
}

impl<A> VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buf: RefCell::new(vec![A::Operand::id(); len + len]),
            def: RefCell::new(vec![A::Operator::id(); len]),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

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

    fn build(&mut self, i: usize) {
        let mut i = self.len + i;
        let mut buf = self.buf.borrow_mut();
        while i > 1 {
            i >>= 1;
            buf[i] =
                A::Operand::op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
    }

    fn apply(&self, i: usize, op: <A::Operator as Magma>::Set) {
        let mut buf = self.buf.borrow_mut();
        let mut def = self.def.borrow_mut();
        A::act(&mut buf[i], op.clone());
        if i < self.len {
            def[i] = A::Operator::op(def[i].clone(), op);
        }
    }

    fn resolve(&self, i: usize) {
        let e = A::Operator::id();
        for h in (1..=(i + 1).next_power_of_two().trailing_zeros() - 1).rev() {
            let i = i >> h;
            let d = std::mem::replace(&mut self.def.borrow_mut()[i], e.clone());
            if &d != &e {
                self.apply(i << 1, d.clone());
                self.apply(i << 1 | 1, d);
            }
        }
    }

    fn resolve_all(&self) {
        let e = A::Operator::id();
        for i in 1..self.len {
            let d = std::mem::replace(&mut self.def.borrow_mut()[i], e.clone());
            self.apply(i, d);
        }
    }
}

impl<A> From<Vec<<A::Operand as Magma>::Set>> for VecActSegtree<A>
where
    A: MonoidAction,
    <A::Operator as Magma>::Set: Clone,
    <A::Operand as Magma>::Set: Clone,
{
    fn from(mut v: Vec<<A::Operand as Magma>::Set>) -> Self {
        let len = v.len();
        let mut buf = vec![A::Operand::id(); len];
        buf.append(&mut v);
        for i in (0..len).rev() {
            buf[i] =
                A::Operand::op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
        let buf = RefCell::new(buf);
        let def = RefCell::new(vec![A::Operator::id(); len]);
        Self { buf, def, len }
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
            return A::Operand::id();
        }
        self.resolve(il);
        self.resolve(ir - 1);
        let mut res_l = A::Operand::id();
        let mut res_r = A::Operand::id();
        let buf = self.buf.borrow();
        while il < ir {
            if il & 1 == 1 {
                let tmp = buf[il].clone();
                res_l = A::Operand::op(res_l, tmp);
                il += 1;
            }
            if ir & 1 == 1 {
                ir -= 1;
                let tmp = buf[ir].clone();
                res_r = A::Operand::op(tmp, res_r);
            }
            il >>= 1;
            ir >>= 1;
        }
        A::Operand::op(res_l, res_r)
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
    }
}

// impl<M> FoldBisect for VecSegtree<M>
// where
//     M: Monoid,
//     M::Set: Clone,
// {
//     fn fold_bisect<F>(&self, l: usize, pred: F) -> (usize, M::Set)
//     where
//         F: Fn(&M::Set) -> bool,
//     {
//         todo!()
//     }
// }

// impl<M> FoldBisectRev for VecSegtree<M>
// where
//     M: Monoid,
//     M::Set: Clone,
// {
//     fn fold_bisect_rev<F>(&self, r: usize, pred: F) -> (usize, M::Set)
//     where
//         F: Fn(&M::Set) -> bool,
//     {
//         todo!()
//     }
// }
