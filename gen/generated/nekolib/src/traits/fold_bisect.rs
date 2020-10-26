//! 区間和の二分探索に関するトレイトたち。
//!
//! 区間のモノイド積が述語を満たすような区間のうち、最大のものを返す。

use super::binop;
use super::fold;

use std::ops::Range;

use binop::{Magma, Monoid};
use fold::Fold;

/// 左端を固定したときの境界を求める。
pub trait FoldBisect: Fold<Range<usize>> {
    /// 添字 `l` と述語 `pred` を引数に取り、次の条件を満たす `r` を返す。
    /// ただし、区間長を `n` とする。
    /// - `pred(&self.fold(l..r))`
    /// - `r == n || !pred(&self.fold(l..r + 1))`
    ///
    /// # Requirements
    /// 対象のモノイドの単位元を `e` とするとき、 `pred(e)` が成り立つ。
    ///
    /// # Examples
    /// ```
    /// use nekolib::ds::VecSegtree;
    /// use nekolib::traits::{Fold, FoldBisect};
    /// use nekolib::utils::OpAdd;
    ///
    /// let vs: VecSegtree<OpAdd<i32>> = vec![2, 4, 1, 3, 5].into();
    ///
    /// assert_eq!(vs.fold_bisect(1, |&x| x < 4), (1_usize, 0));
    /// assert_eq!(vs.fold_bisect(1, |&x| x <= 4), (2_usize, 4));
    /// assert_eq!(vs.fold_bisect(1, |&x| x < 13), (4_usize, 8));
    /// assert_eq!(vs.fold_bisect(1, |&x| x <= 13), (5_usize, 13));
    ///
    /// let l = 1;
    /// let pred = |&x: &i32| x <= 12;
    /// let (r, x) = vs.fold_bisect(l, pred);
    /// assert_eq!(vs.fold(l..r), x);
    /// assert!(pred(&x));
    /// assert!(r == vs.len() || !pred(&vs.fold(l..r + 1)));
    /// ```
    fn fold_bisect<F>(
        &self,
        l: usize,
        pred: F,
    ) -> (usize, <Self::Output as Magma>::Set)
    where
        F: Fn(&<Self::Output as Magma>::Set) -> bool;
}

/// 右端を固定したときの境界を求める。
pub trait FoldBisectRev: Fold<Range<usize>> {
    /// 添字 `r` と述語 `pred` を引数に取り、次の条件を満たす `l` を返す。
    /// - `pred(&self.fold(l..r))`
    /// - `l == 0 || !pred(&self.fold(l - 1..r))`
    ///
    /// # Requirements
    /// 対象のモノイドの単位元を `e` とするとき、`pred(e)` が成り立つ。
    ///
    /// # Examples
    /// ```
    /// use nekolib::ds::VecSegtree;
    /// use nekolib::traits::{Fold, FoldBisectRev};
    /// use nekolib::utils::OpAdd;
    ///
    /// let vs: VecSegtree<OpAdd<i32>> = vec![2, 4, 1, 3, 5].into();
    ///
    /// assert_eq!(vs.fold(..), 15);
    /// assert_eq!(vs.fold_bisect_rev(5, |&x| x <= 0), (5_usize, 0));
    /// assert_eq!(vs.fold_bisect_rev(5, |&x| x < 15), (1_usize, 13));
    /// assert_eq!(vs.fold_bisect_rev(5, |&x| x <= 15), (0_usize, 15));
    ///
    /// let r = 5;
    /// let pred = |&x: &i32| x <= 12;
    /// let (l, x) = vs.fold_bisect_rev(r, pred);
    /// assert_eq!(vs.fold(l..r), x);
    /// assert!(pred(&x));
    /// assert!(l == 0 || !pred(&vs.fold(l - 1..r)));
    /// ```
    fn fold_bisect_rev<F>(
        &self,
        r: usize,
        pred: F,
    ) -> (usize, <Self::Output as Magma>::Set)
    where
        F: Fn(&<Self::Output as Magma>::Set) -> bool;
}
