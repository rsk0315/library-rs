//! 区間和に関するトレイトです。

use std::ops::RangeBounds;

use binop::{Magma, Monoid};

/// 区間和を求める。
pub trait Fold<R: RangeBounds<usize>> {
    type Output: Monoid;
    /// `r` で指定される区間の和を返す。
    fn fold(&self, r: R) -> <Self::Output as Magma>::Set;
}
