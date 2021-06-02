//! 計数クエリ。

use std::ops::RangeBounds;

/// 計数クエリ。
pub trait Count<I> {
    fn count(&self, range: impl RangeBounds<usize>, value: I) -> usize;
}
