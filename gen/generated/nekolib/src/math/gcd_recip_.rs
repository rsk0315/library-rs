//! 最大公約数と逆元。

/// 最大公約数と逆元。
///
/// 次の条件を満たす唯一の $(g, r)$ を返す。
/// - $g = \\gcd(a, b)$
/// - $a\\cdot r \\equiv g \\pmod{b}$
/// - $0\\le r \\lt b/g$
///
/// $a = 0$ のとき $g = b$ であり、$0\\le g \\lt b$ とはならないことに注意せよ。
///
/// # Complexity
/// $O(\\log(\\min\\{a, b\\}))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::gcd_recip;
///
/// let (g, r) = gcd_recip(27, 30);
/// assert_eq!(g, 3);
/// assert_eq!(r, 9);
/// assert_eq!((27 * r) % 30, g % 30);
/// ```
pub fn gcd_recip(a: u64, b: u64) -> (u64, u64) {
    let a = a % b;
    if a == 0 {
        return (b, 0);
    }

    let mut s = b as i64;
    let mut t = a as i64;
    let mut m0 = 0;
    let mut m1 = 1;
    while t > 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        std::mem::swap(&mut s, &mut t);
        std::mem::swap(&mut m0, &mut m1);
    }

    if m0 < 0 {
        m0 += b as i64 / s;
    }
    (s as u64, m0 as u64)
}

#[test]
fn test() {
    for b in 1..=100 {
        for a in 0..b {
            let (g, r) = gcd_recip(a, b);
            assert_eq!(a * r % b, g % b);
        }
    }
}
