//! $n$ 番目の最小値クエリ。

use std::ops::RangeBounds;

/// $n$ 番目の最小値クエリ。
pub trait NthMin {
    type Output;
    fn nth_min(
        &self,
        range: impl RangeBounds<usize>,
        n: usize,
    ) -> Option<Self::Output>;
}
