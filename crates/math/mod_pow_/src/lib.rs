//! 冪乗。

/// 冪乗。
///
/// $x^e \\bmod p$ を返す。
///
/// # Complexity
/// $O(\\log(e))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::mod_pow;
///
/// assert_eq!(mod_pow(3, 14, 10), 9);
/// assert_eq!(mod_pow(2, 11, 1024), 0);
/// assert_eq!(mod_pow(0, 0, 1), 0);
/// ```
pub fn mod_pow(mut x: u64, mut e: u64, p: u64) -> u64 {
    let mut res = 1 % p;
    while e > 0 {
        if e & 1 == 1 {
            res = res * x % p;
        }
        x = x * x % p;
        e >>= 1;
    }
    res
}
