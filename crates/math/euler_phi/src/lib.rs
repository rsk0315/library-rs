//! Euler の $\\varphi$ 関数。

use factors::Factors;

/// Euler の $\\varphi$ 関数。
///
/// $n$ 以下の自然数のうち、$n$ と互いに素であるものの個数を返す。$1$
/// と $1$ は互いに素であることに注意。
///
/// # Note
/// 素数冪 $p^k$, $p\'^{k\'}$ ($p\\ne p\'$) について
/// $\\varphi(p^k p\'^{k\'}) = \\varphi(p^k)\\varphi(p\'^{k\'})$
/// が成り立つ。また、$\\varphi(p^k) = \\varphi^{k-1}\\cdot(p-1)$ が成り立つ。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::EulerPhi;
///
/// assert_eq!(1_u64.euler_phi(), 1);
/// assert_eq!(60_u64.euler_phi(), 16);
/// ```
pub trait EulerPhi {
    fn euler_phi(self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl EulerPhi for $t {
            fn euler_phi(self) -> Self {
                self.factors().map(|(p, e)| p.pow(e - 1) * (p - 1)).product()
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

#[test]
fn test_naive() {
    use gcd::Gcd;
    let n = 100_usize;
    for i in 1..=n {
        let phi = (1..=i).filter(|&j| i.gcd(j) == 1).count();
        assert_eq!(i.euler_phi(), phi);
    }
}
