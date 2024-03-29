//! 配列上の区間に関する関数。

use std::fmt::Debug;
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::{Range, RangeBounds};

/// 区間を配列サイズに収まるように丸める。
///
/// 与えられた区間 `r` と `0..len` の共通部分を、有界な半開区間として返す。
///
/// # Notes
///
/// 終端が陽に与えられたとき（有限のとき）は out of bounds でもそのまま返す。
/// 潜在的なバグの原因を見逃すのを防ぎたいので。
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
    };
    let s_in = match r.start_bound() {
        Included(&s) => s,
        Excluded(&s) => s + 1,
        Unbounded => 0,
    }
    .min(e_ex);
    s_in..e_ex
}

/// 境界チェックを行う。
///
/// # Examples
/// ```
/// use nekolib::utils::check_bounds;
///
/// let a = [0, 1, 2];
/// check_bounds(2, a.len());
/// ```
///
/// ```should_panic
/// use nekolib::utils::check_bounds;
///
/// let a = [0, 1, 2];
/// // panicked at 'index out of bounds: the len is 3 but the index is 3'
/// check_bounds(3, a.len());
/// ```
pub fn check_bounds(i: usize, len: usize) {
    assert!(
        i < len,
        "index out of bounds: the len is {} but the index is {}",
        len,
        i
    );
}

/// 境界チェックを行う。
///
/// # Examples
/// ```
/// use nekolib::utils::check_bounds_range;
///
/// let a = [0, 1, 2];
/// check_bounds_range(2, 0..a.len());
/// check_bounds_range(3, 0..=a.len());
/// ```
///
/// ```should_panic
/// use nekolib::utils::check_bounds_range;
///
/// let a = [0, 1, 2];
/// // panicked at 'index out of bounds: the range is 0..=3 but the index is 4'
/// check_bounds_range(4, 0..=a.len());
/// ```
pub fn check_bounds_range(i: usize, range: impl RangeBounds<usize> + Debug) {
    assert!(
        range.contains(&i),
        "index out of bounds: the range is {:?} but the index is {}",
        range,
        i
    );
}

#[test]
#[should_panic]
fn test_panic_bound_large() { check_bounds_range(4, 0..=3); }

#[test]
#[should_panic]
fn test_panic_bound_small() { check_bounds_range(0, 1..=3); }

#[test]
fn test_check() {
    check_bounds_range(0, 0..=3);
    check_bounds_range(3, 0..=3);
    check_bounds_range(2, 0..3);
}

#[test]
fn test_range() {
    assert_eq!(bounds_within(0..3, 2), 0..3);
    assert_eq!(bounds_within(0..3, 3), 0..3);
    assert_eq!(bounds_within(0..3, 4), 0..3);

    assert_eq!(bounds_within(0.., 2), 0..2);
    assert_eq!(bounds_within(0.., 3), 0..3);
    assert_eq!(bounds_within(0.., 4), 0..4);

    assert_eq!(bounds_within((Excluded(2), Included(5)), 8), 3..6);
    assert_eq!(bounds_within(.., 5), 0..5);
}
