//! $n$ 番目の最小値クエリ。

use std::ops::RangeBounds;

/// $n$ 番目の最小値クエリ。
pub trait Quantile {
    type Output;
    fn quantile(
        &self,
        range: impl RangeBounds<usize>,
        n: usize,
    ) -> Option<Self::Output>;
}
