//! 冪乗。

use const_div::ConstDiv;

/// 冪乗。
///
/// $a^b \\bmod n$ を返す。
///
/// # Complexity
/// $O(\\log(b))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::mod_pow;
///
/// assert_eq!(mod_pow(3, 14, 10), 9);
/// assert_eq!(mod_pow(2, 11, 1024), 0);
/// assert_eq!(mod_pow(0, 0, 1), 0);
/// ```
pub fn mod_pow(a: u64, b: u64, n: u64) -> u64 {
    let cd = ConstDiv::new(n);
    mod_pow_with_cd(cd.rem(a), b, cd)
}

/// 冪乗。
///
/// $a^b \\bmod n$ を返す。
///
/// # Complexity
/// $O(\\log(b))$ time.
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
pub fn mod_pow_with_cd(mut a: u64, mut b: u64, cd: ConstDiv) -> u64 {
    let mut res = cd.rem(1);
    while b > 0 {
        if b & 1 == 1 {
            res = cd.rem(res * a);
        }
        a = cd.rem(a * a);
        b >>= 1;
    }
    res
}

#[test]
fn test() {
    for n in 1..=30 {
        let cd = ConstDiv::new(n);
        for a in 0..30 {
            let mut expected = cd.rem(1);
            for b in 0..30 {
                assert_eq!(mod_pow(a, b, n), expected);
                expected = cd.rem(expected * a);
            }
        }
    }
}
