use std::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};

pub trait StartBounded<T>: RangeBounds<T> {}
pub trait StartInclusive<T>: StartBounded<T> {}
pub trait StartUnbounded<T>: RangeBounds<T> {}
pub trait EndBounded<T>: RangeBounds<T> {}
pub trait EndExclusive<T>: EndBounded<T> {}
pub trait EndInclusive<T>: EndBounded<T> {}
pub trait EndUnbounded<T>: RangeBounds<T> {}

impl<T> StartBounded<T> for Range<T> {}
impl<T> StartBounded<T> for RangeInclusive<T> {}
impl<T> StartBounded<T> for RangeFrom<T> {}
impl<T> StartUnbounded<T> for RangeTo<T> {}
impl<T> StartUnbounded<T> for RangeToInclusive<T> {}
impl<T> StartUnbounded<T> for RangeFull {}

impl<T> StartInclusive<T> for Range<T> {}
impl<T> StartInclusive<T> for RangeInclusive<T> {}
impl<T> StartInclusive<T> for RangeFrom<T> {}

impl<T> EndBounded<T> for Range<T> {}
impl<T> EndBounded<T> for RangeInclusive<T> {}
impl<T> EndUnbounded<T> for RangeFrom<T> {}
impl<T> EndBounded<T> for RangeTo<T> {}
impl<T> EndBounded<T> for RangeToInclusive<T> {}
impl<T> EndUnbounded<T> for RangeFull {}

impl<T> EndExclusive<T> for Range<T> {}
impl<T> EndInclusive<T> for RangeInclusive<T> {}
impl<T> EndExclusive<T> for RangeTo<T> {}
impl<T> EndInclusive<T> for RangeToInclusive<T> {}
