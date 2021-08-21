//! Carmichael の $\\lambda$ 関数。

use factors::Factors;
use lcm::Lcm;

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
/// use nekolib::math::CarmichaelLambda;
///
/// assert_eq!(1_u64.carmichael_lambda(), 1);
/// assert_eq!(15_u64.carmichael_lambda(), 4);
/// assert_eq!(21_u64.carmichael_lambda(), 6);
/// assert_eq!(33_u64.carmichael_lambda(), 10);
/// ```
pub trait CarmichaelLambda {
    fn carmichael_lambda(self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl CarmichaelLambda for $t {
            fn carmichael_lambda(self) -> Self {
                let n = self;
                let e2 = n.trailing_zeros() as $t;
                let mut res = match e2 {
                    0 | 1 => 1,
                    2 => 2,
                    _ => 1 << (e2 - 2),
                };
                for (p, e) in (n >> e2).factors() {
                    res = res.lcm(p.pow(e - 1) * (p - 1));
                }
                res
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

#[test]
fn test() {
    use gcd::Gcd;

    let n_max = 1000_usize;
    assert_eq!(1_usize.carmichael_lambda(), 1);
    for n in 2..=n_max {
        let mut pow = vec![1; n];
        let relp: Vec<_> = (0..n).filter(|&a| a.gcd(n) == 1).collect();
        for e in 1..n {
            for &a in &relp {
                pow[a] = pow[a] * a % n;
            }
            if pow.iter().all(|&x| x == 1) {
                assert_eq!(
                    n.carmichael_lambda(),
                    e,
                    "lambda({}) = {}? {}",
                    n,
                    e,
                    n.carmichael_lambda()
                );
                break;
            }
        }
        assert!(pow.iter().all(|&x| x == 1));
    }
}
