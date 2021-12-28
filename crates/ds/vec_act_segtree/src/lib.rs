//! `Vec` ベースの区間作用セグ木。

use std::cell::RefCell;
use std::ops::{Range, RangeBounds};

use act::Act;
use action::MonoidAction;
use binop::{Identity, Magma};
use buf_range::bounds_within;
use fold::Fold;
use fold_bisect::{FoldBisect, FoldBisectRev};

const WORD_SIZE: u32 = 0_usize.count_zeros();

fn lcp(i: usize, j: usize) -> usize {
    if i == 0 || j == 0 {
        return 0;
    }
    if i == j {
        return i;
    }
    let (i, j) = (i.min(j), i.max(j));
    let iz = i.leading_zeros();
    let i = i << iz;
    let j = j << j.leading_zeros();
    i >> iz.max(WORD_SIZE - (i ^ j).leading_zeros())
}

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
        let id = action.operator().id();
        for i in self.ancestors_upward(start, end).filter(|&i| def[i] == id) {
            buf[i] = operand.op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
    }

    fn apply(&self, i: usize, op: <A::Operator as Magma>::Set) {
        let mut buf = self.buf.borrow_mut();
        let mut def = self.def.borrow_mut();
        buf[i] = self.action.act(buf[i].clone(), op.clone());
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

    fn parent_root(&self, i: usize) -> usize {
        let n = self.len;
        if n.is_power_of_two() {
            return 0;
        }
        let n2 = 2 * n;
        let lsb = n2 & n2.wrapping_neg();
        lcp(i, if i < n2 ^ lsb { n2 ^ lsb } else { n2 })
    }

    fn ancestors_downward(
        &self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = usize> + DoubleEndedIterator {
        self.ancestors_upward(start, end).rev()
    }

    fn ancestors_upward(
        &self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = usize> + DoubleEndedIterator {
        // start <= end

        let mut res = vec![];
        let l = self.len + start;
        let r = self.len + end;
        let pl = self.parent_root(l);
        let pr = self.parent_root(r - 1);

        let mut il = 1;
        let mut ir = 1;
        while l >> il != pl || (r - 1) >> ir != pr {
            if l >> il != pl {
                if l >> il << il != l {
                    res.push(l >> il);
                }
                il += 1;
            }
            if r >> ir != pr {
                if r >> ir << ir != r {
                    res.push((r - 1) >> ir);
                }
                ir += 1;
            }
        }

        res.dedup();
        res.into_iter()
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

    #[cfg(test)]
    pub fn unforced(&self) -> Vec<usize> {
        let def = self.def.borrow();
        let id = self.action.operator().id();
        (0..self.len).filter(|&i| def[i] != id).collect()
    }

    #[cfg(test)]
    pub fn ancestors_sorted(&self, start: usize, end: usize) -> Vec<usize> {
        let mut res: Vec<_> = self.ancestors_upward(start, end).collect();
        res.sort_unstable();
        res
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

#[test]
fn test_ancestors() {
    use op_closure::OpClosure;
    use op_closure_on_op_closure::OpClosureOnOpClosure;

    for n in 1..=100 {
        let mut height = vec![0; n + n];
        for i in n..n + n {
            height[i] = 1;
        }
        for i in (1..n).rev() {
            if height[i << 1] == height[i << 1 | 1] {
                height[i] = height[i << 1] + 1;
            }
        }

        let action = OpClosureOnOpClosure::new(
            OpClosure::new(|_, _| (), || ()),
            OpClosure::new(|_, _| (), || ()),
            |_, _| (),
        );
        let st: VecActSegtree<_> = (vec![(); n], action).into();
        for l in 0..n {
            for r in l..n {
                let arch = {
                    let mut arch = vec![];
                    let mut l = n + l;
                    let mut r = n + r + 1;
                    while l < r {
                        if l & 1 == 1 {
                            arch.push(l);
                            l += 1;
                        }
                        if r & 1 == 1 {
                            r -= 1;
                            arch.push(r);
                        }
                        l >>= 1;
                        r >>= 1;
                    }
                    arch
                };

                let mut expected = vec![];
                for mut i in arch {
                    i >>= 1;
                    while height[i] != 0 {
                        expected.push(i);
                        i >>= 1;
                    }
                }
                expected.sort_unstable();
                expected.dedup();

                let actual = st.ancestors_sorted(l, r + 1);
                assert_eq!(actual, expected);
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
fn dump(n: usize, node: &[usize]) {
    use std::collections::BTreeSet;
    let node: BTreeSet<_> = node.iter().copied().collect();

    let len = n.next_power_of_two() << 1;
    for i in 1..n + n {
        let shift = WORD_SIZE - 1 - i.leading_zeros();
        let ch = if node.contains(&i) { "o" } else { "-" };
        eprint!("[{}]", ch.repeat((len >> shift) - 2));
        if i + 1 == n + n || (i + 1).is_power_of_two() {
            eprintln!();
        }
    }
}
