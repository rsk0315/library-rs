//! 最大公約数。

/// 最大公約数。
///
/// # Complexity
/// $O(\\log(\\min\\{m, n\\}))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::gcd;
///
/// assert_eq!(gcd(12, 18), 6);
/// assert_eq!(gcd(0, 3), 3);
/// assert_eq!(gcd(0, 0), 0);
/// ```
pub fn gcd(mut m: u128, mut n: u128) -> u128 {
    while n > 0 {
        let tmp = m % n;
        m = std::mem::replace(&mut n, tmp);
    }
    m
}
