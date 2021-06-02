//! 計数クエリ。

use std::ops::RangeBounds;

/// 計数クエリ。
pub trait Count<I> {
    fn count(&self, range: impl RangeBounds<usize>, value: I) -> usize;
}

/// 三方向計数クエリ。
pub trait Count3way<I> {
    fn count_3way(
        &self,
        range: impl RangeBounds<usize>,
        value: I,
    ) -> Count3wayResult;
}

pub struct Count3wayResult {
    lt: usize,
    eq: usize,
    gt: usize,
}

impl Count3wayResult {
    pub fn new(lt: usize, eq: usize, gt: usize) -> Self { Self { lt, eq, gt } }
    pub fn lt(&self) -> usize { self.lt }
    pub fn eq(&self) -> usize { self.eq }
    pub fn gt(&self) -> usize { self.gt }
    pub fn le(&self) -> usize { self.lt + self.eq }
    pub fn ge(&self) -> usize { self.gt + self.eq }
}
