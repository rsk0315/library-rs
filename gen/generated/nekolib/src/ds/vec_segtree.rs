//! `Vec` ベースのセグ木。

use super::super::traits::binop;
use super::super::traits::fold;
use super::super::traits::fold_bisect;
use super::super::traits::get_mut;
use super::super::traits::set_value;
use super::super::utils::buf_range;

use std::convert::From;
use std::fmt::{self, Debug};
use std::iter::{IntoIterator, Iterator};
use std::ops::{Deref, DerefMut, Index, Range, RangeBounds};

use binop::Monoid;
use buf_range::{bounds_within, check_bounds, check_bounds_range};
use fold::Fold;
use fold_bisect::{FoldBisect, FoldBisectRev};
use get_mut::GetMut;
use set_value::SetValue;

/// `Vec` ベースのセグ木。
///
/// 非再帰実装かつ配列サイズを $2n$ とするセグ木。
/// モノイドを対象として要素の取得・更新および任意区間のモノイド積を処理する。
/// 加えて、モノイド積を引数とする述語に対して、それが真となる区間の境界を求められる。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`, `from`|$\\Theta(n)$|
/// |`index`|$\\Theta(1)$|
/// |`set_value`|$\\Theta(\\log(n))$|
/// |区間 $[l, r)$ の `fold`|$\\Theta(\\log(r-l))$|
/// |`fold_bisect`, `fold_bisect_rev`|$O(\\log(n))$|
///
/// # Examples
/// ```
/// use nekolib::ds::VecSegtree;
/// use nekolib::traits::{Fold, FoldBisect, FoldBisectRev, GetMut};
/// use nekolib::utils::OpAdd;
///
/// let mut vs: VecSegtree<OpAdd<i32>> = vec![2, 4, 7, 3, 5].into();
/// assert_eq!(vs.fold(1..3), 11);
/// assert_eq!(vs.fold(..), 21);
///
/// *vs.get_mut(2).unwrap() = 1; // [2, 4, 1, 3, 5]
/// assert_eq!(vs.fold(1..3), 5);
/// assert_eq!(vs.fold(1..=3), 8);
/// assert_eq!(vs.fold_bisect(1, |&x| x < 4), (1_usize, 0));
/// assert_eq!(vs.fold_bisect(1, |&x| x <= 4), (2_usize, 4));
/// assert_eq!(vs.fold_bisect(1, |&x| x < 13), (4_usize, 8));
/// assert_eq!(vs.fold_bisect(1, |&x| x <= 13), (5_usize, 13));
///
/// assert_eq!(vs.fold(..), 15);
/// assert_eq!(vs.fold_bisect_rev(5, |&x| x <= 0), (5_usize, 0));
/// assert_eq!(vs.fold_bisect_rev(5, |&x| x < 15), (1_usize, 13));
/// assert_eq!(vs.fold_bisect_rev(5, |&x| x <= 15), (0_usize, 15));
///
/// let l = 1;
/// let pred = |&x: &i32| x <= 12;
/// let (r, x) = vs.fold_bisect(l, pred);
/// assert_eq!(vs.fold(l..r), x);
/// assert!(pred(&x));
/// assert!(r == vs.len() || !pred(&vs.fold(l..r + 1)));
///
/// let r = 5;
/// let pred = |&x: &i32| x <= 12;
/// let (l, x) = vs.fold_bisect_rev(r, pred);
/// assert_eq!(vs.fold(l..r), x);
/// assert!(pred(&x));
/// assert!(l == 0 || !pred(&vs.fold(l - 1..r)));
/// ```
#[derive(Clone)]
pub struct VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    buf: Vec<M::Set>,
    len: usize,
    monoid: M,
}

impl<M> VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    #[must_use]
    pub fn new(len: usize) -> Self
    where
        M: Default,
    {
        let monoid = M::default();
        Self { len, buf: vec![monoid.id(); len + len], monoid }
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
}

impl<M, B> Fold<B> for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
    B: RangeBounds<usize>,
{
    type Output = M;
    fn fold(&self, b: B) -> M::Set {
        let Range { start, end } = bounds_within(b, self.len);
        let mut il = self.len + start;
        let mut ir = self.len + end;
        let mut res_l = self.monoid.id();
        let mut res_r = self.monoid.id();
        while il < ir {
            if il & 1 == 1 {
                res_l = self.monoid.op(res_l, self.buf[il].clone());
                il += 1;
            }
            if ir & 1 == 1 {
                ir -= 1;
                res_r = self.monoid.op(self.buf[ir].clone(), res_r);
            }
            il >>= 1;
            ir >>= 1;
        }
        self.monoid.op(res_l, res_r)
    }
}

impl<M> SetValue<usize> for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    type Input = M::Set;
    fn set_value(&mut self, i: usize, x: Self::Input) {
        check_bounds(i, self.len);
        *self.get_mut(i).unwrap() = x;
    }
}

#[doc(hidden)]
pub struct GetMutIndex<'a, M>
where
    M: Monoid,
    M::Set: Clone,
{
    tree: &'a mut VecSegtree<M>,
    index: usize,
}

impl<'a, M: 'a> GetMut<'a> for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    type Output = GetMutIndex<'a, M>;
    fn get_mut(&'a mut self, index: usize) -> Option<GetMutIndex<'a, M>> {
        if index < self.len {
            Some(GetMutIndex { tree: self, index })
        } else {
            None
        }
    }
}

impl<M> Drop for GetMutIndex<'_, M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn drop(&mut self) {
        let mut i = self.tree.len + self.index;
        while i > 1 {
            i >>= 1;
            self.tree.buf[i] = self.tree.monoid.op(
                self.tree.buf[i << 1].clone(),
                self.tree.buf[i << 1 | 1].clone(),
            );
        }
    }
}

impl<M> Deref for GetMutIndex<'_, M>
where
    M: Monoid,
    M::Set: Clone,
{
    type Target = M::Set;
    fn deref(&self) -> &Self::Target {
        &self.tree.buf[self.tree.len + self.index]
    }
}

impl<M> DerefMut for GetMutIndex<'_, M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree.buf[self.tree.len + self.index]
    }
}

impl<M> Default for VecSegtree<M>
where
    M: Monoid + Default,
    M::Set: Clone,
{
    fn default() -> Self { Self { buf: vec![], len: 0, monoid: M::default() } }
}

impl<M> From<Vec<M::Set>> for VecSegtree<M>
where
    M: Monoid + Default,
    M::Set: Clone,
{
    fn from(v: Vec<M::Set>) -> Self { Self::from((v, M::default())) }
}

impl<M> From<(Vec<M::Set>, M)> for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn from((mut v, monoid): (Vec<M::Set>, M)) -> Self {
        let len = v.len();
        let mut buf = vec![monoid.id(); len];
        buf.append(&mut v);
        for i in (0..len).rev() {
            buf[i] = monoid.op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
        Self { buf, len, monoid }
    }
}

impl<M> Index<usize> for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    type Output = M::Set;
    fn index(&self, i: usize) -> &Self::Output {
        check_bounds(i, self.len);
        &self.buf[i + self.len]
    }
}

impl<M> From<VecSegtree<M>> for Vec<M::Set>
where
    M: Monoid,
    M::Set: Clone,
{
    fn from(v: VecSegtree<M>) -> Self {
        v.buf.into_iter().skip(v.len).collect()
    }
}

impl<M> Debug for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.buf[self.len..].iter()).finish()
    }
}

impl<M> FoldBisect for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn fold_bisect<F>(&self, l: usize, pred: F) -> (usize, M::Set)
    where
        F: Fn(&M::Set) -> bool,
    {
        check_bounds_range(l, 0..=self.len);

        let mut x = self.monoid.id();
        assert!(pred(&x), "`pred(id)` must hold");
        match self.fold(l..) {
            x if pred(&x) => return (self.len, x),
            _ => {}
        }

        for v in self.nodes(l, self.len) {
            let tmp = self.monoid.op(x.clone(), self.buf[v].clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                v <<= 1;
                let tmp = self.monoid.op(x.clone(), self.buf[v].clone());
                if pred(&tmp) {
                    x = tmp;
                    v += 1;
                }
            }
            return (v - self.len, x);
        }
        unreachable!();
    }
}

impl<M> FoldBisectRev for VecSegtree<M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn fold_bisect_rev<F>(&self, r: usize, pred: F) -> (usize, M::Set)
    where
        F: Fn(&M::Set) -> bool,
    {
        check_bounds_range(r, 0..=self.len);

        let mut x = self.monoid.id();
        assert!(pred(&x), "`pred(id)` must hold");
        match self.fold(..r) {
            x if pred(&x) => return (0, x),
            _ => {}
        }

        for v in self.nodes_rev(0, r) {
            let tmp = self.monoid.op(self.buf[v].clone(), x.clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                v = v << 1 | 1;
                let tmp = self.monoid.op(self.buf[v].clone(), x.clone());
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
