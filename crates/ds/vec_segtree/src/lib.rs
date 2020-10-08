//! `Vec` ベースのセグ木。

use std::convert::From;
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, RangeBounds};

use binop::{Magma, Monoid};
use buf_range::bounds_within;
use fold::Fold;
use fold_bisect::{FoldBisect, FoldBisectRev};
use set_value::SetValue;

/// `Vec` ベースのセグ木。
///
/// 非再帰実装かつ配列サイズを $2n$ とするセグ木。
/// モノイド `M` を対象として要素の取得・更新および区間和を処理する。
/// 加えて、述語 `F` に対する二分探索も処理できる。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`, `from`|$\\Theta(n)$|
/// |`index`|$\\Theta(1)$|
/// |`set_value`|$\\Theta(\\log(n))$|
/// |$[l, r)$ の `fold`|$\\Theta(\\log(r-l))$|
/// |`fold_bisect`, `fold_bisect_rev`|$O(\\log(n))$|
///
/// # Examples
/// ```
/// use nekolib::ds::VecSegtree;
/// use nekolib::traits::{Fold, FoldBisect, FoldBisectRev, SetValue};
/// use nekolib::utils::OpAdd;
///
/// let mut vs: VecSegtree<OpAdd<i32>> = vec![2, 4, 7, 3, 5].into();
/// assert_eq!(vs.fold(1..3), 11);
/// assert_eq!(vs.fold(..), 21);
///
/// vs.set_value(2usize, 1); // [2, 4, 1, 3, 5]
/// assert_eq!(vs.fold(1..3), 5);
/// assert_eq!(vs.fold(1..=3), 8);
/// assert_eq!(vs.fold_bisect(1, |&x| x < 4), Some(1));
/// assert_eq!(vs.fold_bisect(1, |&x| x <= 4), Some(2));
/// assert_eq!(vs.fold_bisect(1, |&x| x < 13), Some(4));
/// assert_eq!(vs.fold_bisect(1, |&x| x <= 13), None);
///
/// assert_eq!(vs.fold(..), 15);
/// assert_eq!(vs.fold_bisect_rev(5, |&x| x <= 0), Some(4));
/// assert_eq!(vs.fold_bisect_rev(5, |&x| x < 15), Some(0));
/// assert_eq!(vs.fold_bisect_rev(5, |&x| x <= 15), None);
/// ```
#[derive(Clone)]
pub struct VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    buf: Vec<<M as Magma>::Set>,
    len: usize,
}

impl<M> VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buf: vec![M::id(); len + len],
        }
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
}

impl<M, B> Fold<B> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
    B: RangeBounds<usize>,
{
    type Output = M;
    fn fold(&self, b: B) -> <M as Magma>::Set {
        let b = bounds_within(b, self.len);
        let mut il = self.len + b.start;
        let mut ir = self.len + b.end;
        let mut resl = M::id();
        let mut resr = M::id();
        while il < ir {
            if il & 1 == 1 {
                resl = <M as Magma>::op(resl, self.buf[il].clone());
                il += 1;
            }
            if ir & 1 == 1 {
                ir -= 1;
                resr = <M as Magma>::op(self.buf[ir].clone(), resr);
            }
            il >>= 1;
            ir >>= 1;
        }
        <M as Magma>::op(resl, resr)
    }
}

impl<M> SetValue<usize> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    type Input = <M as Magma>::Set;
    fn set_value(&mut self, i: usize, x: Self::Input) {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );

        let mut i = i + self.len;
        self.buf[i] = x;
        while i > 1 {
            i >>= 1;
            self.buf[i] = <M as Magma>::op(
                self.buf[i << 1].clone(),
                self.buf[i << 1 | 1].clone(),
            );
        }
    }
}

impl<M> From<Vec<<M as Magma>::Set>> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    fn from(mut v: Vec<<M as Magma>::Set>) -> Self {
        let len = v.len();
        let mut buf = vec![M::id(); len];
        buf.append(&mut v);
        for i in (0..len).rev() {
            buf[i] =
                <M as Magma>::op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
        Self { buf, len }
    }
}

impl<M> Index<usize> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    type Output = <M as Magma>::Set;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );

        &self.buf[i + self.len]
    }
}

impl<M> From<VecSegtree<M>> for Vec<<M as Magma>::Set>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    fn from(v: VecSegtree<M>) -> Self {
        v.buf.into_iter().skip(v.len).collect()
    }
}

impl<M> FoldBisect for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    type Folded = M;
    fn fold_bisect<F>(&self, l: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<M as Magma>::Set) -> bool,
    {
        assert!(
            l < self.len,
            "index out of bounds: the len is {} but the index is {}; valid range: 0..{}",
            self.len, l, self.len
        );

        let mut x = M::id();
        if !pred(&x) {
            return Some(l);
        } else if pred(&self.fold(l..)) {
            return None;
        }

        for v in self.nodes(l, self.len) {
            let tmp = <M as Magma>::op(x.clone(), self.buf[v].clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                v <<= 1;
                let tmp = <M as Magma>::op(x.clone(), self.buf[v].clone());
                if pred(&tmp) {
                    x = tmp;
                    v += 1;
                }
            }
            return Some(v - self.len);
        }
        unreachable!();
    }
}

impl<M> FoldBisectRev for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    type Folded = M;
    fn fold_bisect_rev<F>(&self, r: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<M as Magma>::Set) -> bool,
    {
        assert!(
            r <= self.len,
            "index out of bounds: the len is {} but the index is {}; valid range: 0..={}",
            self.len, r, self.len
        );

        let mut x = M::id();
        if !pred(&x) {
            return Some(r);
        } else if pred(&self.fold(..r)) {
            return None;
        }

        for v in self.nodes(0, r).into_iter().rev() {
            let tmp = <M as Magma>::op(self.buf[v].clone(), x.clone());
            if pred(&tmp) {
                x = tmp;
                continue;
            }
            let mut v = v;
            while v < self.len {
                v <<= 1;
                v += 1;
                let tmp = <M as Magma>::op(self.buf[v].clone(), x.clone());
                if pred(&tmp) {
                    x = tmp;
                    v -= 1;
                }
            }
            return Some(v - self.len);
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use op_add::OpAdd;

    use crate::*;

    #[test]
    fn test() {
        let mut s = VecSegtree::<OpAdd<i32>>::new(5);
        s.set_value(1usize, 3);
        s.set_value(4usize, 5);
        s.set_value(2usize, 2);
        s.set_value(0usize, 4);
        assert_eq!(Vec::<_>::from(s.clone()), vec![4, 3, 2, 0, 5]);
        assert_eq!(s.fold(0..4), 9);
        assert_eq!(s.fold(1..=3), 5);
        assert_eq!(s.fold(3..), 5);
        assert_eq!(s.fold(..0), 0);
        assert_eq!(s.fold(..=2), 9);
        assert_eq!(s.fold(..), 14);
        assert_eq!(s[1], 3);

        let s = VecSegtree::<OpAdd<i32>>::from(vec![1, 4, 2, 5, 3]);
        assert_eq!(Vec::<_>::from(s.clone()), vec![1, 4, 2, 5, 3]);
        assert_eq!(s.fold(1..3), 6);
        assert_eq!(s.fold(0..=2), 7);
        assert_eq!(s.fold(5..), 0);
        assert_eq!(s.fold(..3), 7);
        assert_eq!(s.fold(..=4), 15);
        assert_eq!(s.fold(..), 15);
        assert_eq!(s[4], 3);
    }

    #[test]
    #[should_panic(expected = "index")]
    fn test_out_of_index() {
        let mut s = VecSegtree::<OpAdd<i32>>::from(vec![1, 4, 2, 5, 3]);
        s.set_value(5, 2);
    }
}
