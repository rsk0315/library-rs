//! 最小公倍数。

use super::gcd_;

use gcd_::gcd;

/// 最小公倍数。
///
/// # Complexity
/// $O(\\log(\\min\\{m, n\\}))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::lcm;
///
/// assert_eq!(lcm(12, 18), 36);
/// assert_eq!(lcm(0, 3), 0);
/// assert_eq!(lcm(0, 0), 0);
/// ```
pub fn lcm(m: u128, n: u128) -> u128 {
    if m == 0 || n == 0 {
        0
    } else {
        m / gcd(m, n) * n
    }
}

/// オーバーフロー検出つき最小公倍数。
///
/// # Complexity
/// $O(\\log(\\min\\{m, n\\}))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::{lcm, overflowing_lcm};
///
/// assert_eq!(overflowing_lcm(12, 18), (36, false));
/// assert!(overflowing_lcm(2_u128.pow(60), 3_u128.pow(50)).1);
/// ```
pub fn overflowing_lcm(m: u128, n: u128) -> (u128, bool) {
    if m == 0 || n == 0 {
        (0, false)
    } else {
        (m / gcd(m, n)).overflowing_mul(n)
    }
}
