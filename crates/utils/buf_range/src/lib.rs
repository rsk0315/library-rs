//! 配列上の区間に関する関数。

use std::ops::Bound::*;
use std::ops::{Range, RangeBounds};

/// 区間を配列サイズに収まるように丸める。
///
/// 与えられた区間 `r` と `0..len` の共通部分を、有界な半開区間として返す。
///
/// # Examples
/// ```
/// use nekolib::utils::bounds_within;
///
/// assert_eq!(bounds_within(.., 7), 0..7);
/// assert_eq!(bounds_within(..=4, 7), 0..5);
/// ```
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
