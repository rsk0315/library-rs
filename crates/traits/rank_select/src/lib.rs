//! rank/select クエリ。

use std::ops::RangeBounds;

/// rank/select クエリ。
pub trait RankSelect {
    /// 計数の対象となる型。
    type Input: Eq;

    /// 区間 `r` 中の `x` の個数を返す。
    fn rank(&self, r: impl RangeBounds<usize>, x: Self::Input) -> usize;

    /// 区間 `r` の始点を `Included(s)` としたとき、`rank(s..e, x)` が `k` となるような
    /// `e` の最小値を返す。直感的には、`k` 個目の `x` を探すのに対応する。
    fn select(
        &self,
        r: impl RangeBounds<usize>,
        x: Self::Input,
        k: usize,
    ) -> Option<usize>;
}
