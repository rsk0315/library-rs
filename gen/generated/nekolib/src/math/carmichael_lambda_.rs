//! Carmichael の $\\lambda$ 関数。

use super::factors_;

use factors_::factors;

/// Carmichael の $\\lambda$ 関数。
///
/// $\\lambda(n)$ は、$\\gcd(a, n)$ である任意の $a$ に対して
/// $a^m\\equiv 1 \\pmod{n}$ となる最小の $m$ として定義される。
///
/// $\\gdef{\\lcm}{\\operatorname\*{lcm}}$
/// 以下の式によって計算される。
/// - $\\lambda(1) = 1$
/// - $\\lambda(2) = 1$
/// - $\\lambda(4) = 2$
/// - $\\lambda(2^e) = 2^{e-2}$ for $e\\ge 3$
/// - $\\lambda(p^e) = \\varphi(p^e) = p^{e-1}(p-1)$ for odd prime $p$
/// - $\\lambda(\\prod\_{p:\\text{ prime}} p\_i^{e\_i}) = \\lcm\_i \\lambda(p\_i^{e\_i})$
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::carmichael_lambda;
///
/// assert_eq!(carmichael_lambda(1), 1);
/// assert_eq!(carmichael_lambda(15), 4);
/// assert_eq!(carmichael_lambda(21), 6);
/// assert_eq!(carmichael_lambda(33), 10);
/// ```
pub fn carmichael_lambda(n: u64) -> u64 {
    let e2 = n.trailing_zeros() as u64;
    let mut res = match e2 {
        0 | 1 => 1,
        2 => 2,
        _ => 1 << (e2 - 2),
    };
    for (p, e) in factors(n >> e2) {
        res = lcm(res, p.pow(e - 1) * (p - 1));
    }
    res
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let tmp = a % b;
        a = std::mem::replace(&mut b, tmp);
    }
    a
}

// (a, b) != (0, 0)
fn lcm(a: u64, b: u64) -> u64 { a / gcd(a, b) * b }

#[test]
fn test() {
    let n_max = 1000;
    assert_eq!(carmichael_lambda(1), 1);
    for n in 2..=n_max {
        let mut pow = vec![1; n as usize];
        let relp: Vec<_> = (0..n).filter(|&a| gcd(a, n) == 1).collect();
        for e in 1..n {
            for &a in &relp {
                pow[a as usize] = pow[a as usize] * a % n;
            }
            if pow.iter().all(|&x| x == 1) {
                assert_eq!(carmichael_lambda(n), e);
                break;
            }
        }
        assert!(pow.iter().all(|&x| x == 1));
    }
}
