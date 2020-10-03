use std::ops::Bound::*;
use std::ops::{Range, RangeBounds};

pub fn bounds_within<R: RangeBounds<usize>>(r: R, len: usize) -> Range<usize> {
    let e_ex = match r.end_bound() {
        Included(&e) => e + 1,
        Excluded(&e) => e,
        Unbounded => len,
    }
    .min(len);
    let s_in = match r.start_bound() {
        Included(&s) => s,
        Excluded(&s) => s + 1,
        Unbounded => 0,
    }
    .min(e_ex);
    s_in..e_ex
}
