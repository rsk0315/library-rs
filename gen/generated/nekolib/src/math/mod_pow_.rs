//! 冪乗。

use super::const_div;

use const_div::ConstDiv;

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
pub fn mod_pow(x: u64, e: u64, p: u64) -> u64 {
    let cd = ConstDiv::new(p);
    mod_pow_with_cd(x % p, e, cd)
}

/// 冪乗。
///
/// $x^e \\bmod p$ を返す。
///
/// # Complexity
/// $O(\\log(e))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::{mod_pow_with_cd, ConstDiv};
///
/// let cd = ConstDiv::new(13);
///
/// assert_eq!(mod_pow_with_cd(3, 14, cd), 9);
/// assert_eq!(mod_pow_with_cd(2, 11, cd), 7);
/// assert_eq!(mod_pow_with_cd(0, 0, cd), 1);
/// ```
pub fn mod_pow_with_cd(mut x: u64, mut e: u64, cd: ConstDiv) -> u64 {
    let mut res = cd.rem(1);
    while e > 0 {
        if e & 1 == 1 {
            res = cd.rem(res * x);
        }
        x = cd.rem(x * x);
        e >>= 1;
    }
    res
}

#[test]
fn test() {
    let cd = ConstDiv::new(13);
    assert_eq!(mod_pow_with_cd(2, 11, cd), 7);
    assert_eq!(mod_pow_with_cd(0, 0, cd), 1);
    assert_eq!(mod_pow(0, 0, 1), 0);
}
