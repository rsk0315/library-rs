//! $n$ 番目の出現位置クエリ。

use std::ops::RangeBounds;

/// $n$ 番目の出現位置クエリ。
pub trait FindNth<I> {
    fn find_nth(
        &self,
        range: impl RangeBounds<usize>,
        value: I,
        n: usize,
    ) -> Option<usize>;
}
