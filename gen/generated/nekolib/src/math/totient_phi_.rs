//! Euler の $\\phi$ 関数。

use super::factors_;

use factors_::factors;

/// Euler の $\\phi$ 関数。
///
/// $n$ 以下の自然数のうち、$n$ と互いに素であるものの個数を返す。$1$
/// と $1$ は互いに素であることに注意。
///
/// # Note
/// 素数冪 $p^k$, $p'^{k'}$ ($p\\ne p'$) について $\\phi(p^k p'^{k'} = \\phi(p^k)\\phi(p'^{k'})$
/// が成り立つ。また、$\\phi(p^k) = \\phi^{k-1}\\cdot(p-1)$ が成り立つ。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::totient_phi;
///
/// assert_eq!(totient_phi(1), 1);
/// assert_eq!(totient_phi(60), 16);
/// ```
pub fn totient_phi(n: u64) -> u64 {
    factors(n).map(|(p, e)| p.pow(e - 1) * (p - 1)).product()
}

#[test]
fn test_naive() {
    use gcd_::gcd;
    let n = 100;
    for i in 1..=n {
        let phi =
            (1..=i).filter(|&j| gcd(i as u128, j as u128) == 1).count() as u64;
        assert_eq!(totient_phi(i), phi);
    }
}
