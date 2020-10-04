use std::ops::RangeBounds;

use binop::{Magma, Monoid};

pub trait Fold<R: RangeBounds<usize>> {
    type Output: Monoid;
    fn fold(&self, r: R) -> <Self::Output as Magma>::Set;
}
