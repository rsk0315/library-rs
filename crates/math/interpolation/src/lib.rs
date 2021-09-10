//! Lagrange 補間。

use const_div::ConstDiv;
use gcd_recip::GcdRecip;

/// Lagrange 補間。
///
/// 与えられた $\\langle f(0), f(1), \\dots, f(n-1)\\rangle$ に対して $\\hat{f}(i)=f(i)$
/// ($i=0,1,\\dots,n-1$) なる $n$ 次多項式 $\\hat{f}$ を考える。
/// この $\\hat{f}$ に対して $\\hat{f}(x)\\bmod p$ を返す。
///
/// # Idea
/// `todo!()`
///
/// # Complexity
/// 前処理に $O(n)$ 時間、$\\hat{f}(x)$ を求めるのに $O(n)$ 時間。
///
/// # Examples
/// ```
/// use nekolib::math::Interpolation;
///
/// let f = Interpolation::with(vec![0, 1, 3], 998244353);
/// assert_eq!(f.interpolate(0), 0);
/// assert_eq!(f.interpolate(3), 6);
/// assert_eq!(f.interpolate(4), 10);
/// assert_eq!(f.interpolate(100000000), 722404071);
/// ```
///
/// # See also
/// - <https://rsk0315.hatenablog.com/entry/2019/04/25/141012>
pub struct Interpolation {
    first: Vec<u64>,
    fact_recip: Vec<u64>,
    cd: ConstDiv,
    modulo: u64,
}

impl Interpolation {
    pub fn with(first: Vec<u64>, modulo: u64) -> Self {
        let n = first.len();
        let cd = ConstDiv::new(modulo);
        let r = (2..n as u64).reduce(|x, y| cd.rem(x * y)).unwrap_or(1);
        let mut fact_recip = vec![1; n];
        fact_recip[n - 1] = r.gcd_recip(modulo).1;
        for i in (2..n).rev() {
            fact_recip[i - 1] = cd.rem(fact_recip[i] * i as u64);
        }
        Self { first, fact_recip, cd, modulo }
    }
    pub fn interpolate(&self, x: u64) -> u64 {
        if (x as usize) < self.first.len() {
            return self.first[x as usize];
        }
        let cd = self.cd;
        let modulo = self.modulo;
        let n = self.first.len() - 1;
        // omega = (x-0) * ... (x-n)
        let omega = (0..=n as u64)
            .map(|i| cd.rem(x + modulo - i))
            .reduce(|acc, x| cd.rem(acc * x))
            .unwrap();
        let sigma: u64 = (0..=n)
            .map(|i| {
                let wi = cd.rem(self.fact_recip[i] * self.fact_recip[n - i]);
                let sgn = if (n - i) % 2 != 0 { modulo - wi } else { wi };
                let tmp = cd.rem(self.first[i] * sgn);
                tmp * (x + modulo - i as u64).gcd_recip(modulo).1
            })
            .sum();
        cd.rem(omega * cd.rem(sigma))
    }
}
