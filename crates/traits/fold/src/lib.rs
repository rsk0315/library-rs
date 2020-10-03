use std::ops::RangeBounds;

use binop::Monoid;

pub trait Fold<R: RangeBounds<usize>> {
    type Output: Monoid;
    fn fold<R>(r: R) -> Self::Output::Set;
}
