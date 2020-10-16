//! 区間和の二分探索に関するトレイトたちです。
//!
//! # Suggestions
//! 設計を見直します。
//! `r = fold_bisect(l, pred).unwrap()` および `l = fold_bisect_rev(r, pred).unwrap()` の
//! どちらの場合においても、`fold(l..r)` が `true`（あるいは `false`）で
//! 同じ値になる方がいいんじゃないかという気がしてきました。
//!
//! 現状では、前者は `fold(l..=r)` で（返り値の `r` を区間に含むので）`false`、
//! 後者は `fold(l..r)` で（返り値の `l` を区間に含むので）`false` 、
//! それより短い区間であれば `true` ということになっています。

use binop::{Magma, Monoid};

/// 左端を固定したときの区間和に関する境界を求める。
pub trait FoldBisect {
    /// 区間和に関するモノイド $(M, \\circ, e)$。
    type Folded: Monoid;
    /// # Examples
    /// ```
    /// use nekolib::ds::VecSegtree;
    /// use nekolib::traits::{Fold, FoldBisect};
    /// use nekolib::utils::OpMin;
    ///
    /// let vs: VecSegtree<OpMin<i32>> = vec![5, 3, 7, 4, 2].into();
    /// let i = 1;
    /// let pred = |&x: &i32| x >= 4;
    /// let j = vs.fold_bisect(i, pred).unwrap();
    /// // assert!(pred(&vs.fold(i..j)));
    /// // assert!(!pred(&vs.fold(i..=j)));
    /// ```
    fn fold_bisect<F>(&self, i: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<Self::Folded as Magma>::Set) -> bool;
}

/// 右端を固定したときの区間和に関する境界を求める。
pub trait FoldBisectRev {
    /// 区間和に関するモノイド $(M, \\circ, e)$。
    type Folded: Monoid;
    /// # Examples
    /// ```
    /// use nekolib::ds::VecSegtree;
    /// use nekolib::traits::{Fold, FoldBisectRev};
    /// use nekolib::utils::OpMin;
    ///
    /// let vs: VecSegtree<OpMin<i32>> = vec![5, 3, 7, 4, 2].into();
    /// let i = 4;
    /// let pred = |&x: &i32| x >= 4;
    /// let j = vs.fold_bisect_rev(i, pred).unwrap();
    /// // assert!(pred(&vs.fold(i+1..j)));
    /// // assert!(!pred(&vs.fold(i..j)));
    /// ```
    fn fold_bisect_rev<F>(&self, i: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<Self::Folded as Magma>::Set) -> bool;
}
