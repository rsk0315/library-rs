use std::ops::Range;

pub trait PatSearcher {
    type Item: Clone + Eq;
    type Output;
    fn with<P: AsRef<[Self::Item]>>(pat: P) -> Self;
    fn occurrences(&self, text: &[Self::Item]) -> Self::Output;
}
