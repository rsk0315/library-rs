//! 法 $p$ での二項係数。

use mod_recip_table_::mod_recip_table_prime;

/// 法 $p$ での二項係数。
///
/// # Examples
/// ```
/// use nekolib::math::ModFactorialBinom;
///
/// const MOD: u64 = 998244353;
/// let mfb = ModFactorialBinom::new(10, MOD);
///
/// assert_eq!(mfb.factorial(5), 120);
/// assert_eq!(mfb.factorial_recip(5), 856826403);
/// assert_eq!(mfb.recip(3), 332748118);
///
/// assert_eq!(mfb.perm(6, 4), 360);
/// assert_eq!(mfb.binom(7, 3), 35);
/// ```
///
/// ```should_panic
/// use nekolib::math::ModFactorialBinom;
///
/// const MOD: u64 = 998244353;
/// let mfb = ModFactorialBinom::new(10, MOD);
///
/// mfb.recip(0);
/// ```
pub struct ModFactorialBinom {
    p: u64,
    fact: Vec<u64>,
    fact_recip: Vec<u64>,
}

impl ModFactorialBinom {
    /// $(0!, 1!, \\dots, n!)$ と $(0!^{-1}, 1!^{-1}, \\dots, n!^{-1})$ を
    /// $p$ を法として計算する。
    pub fn new(n: usize, p: u64) -> Self {
        let mut fact = vec![1; n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as u64 % p;
        }
        let recip = mod_recip_table_prime(n as u64, p);
        let mut fact_recip = vec![1; n + 1];
        for i in 2..=n {
            fact_recip[i] = fact_recip[i - 1] * recip[i] % p;
        }
        Self { p, fact, fact_recip }
    }

    /// $i! \\bmod p$ を返す。
    pub fn factorial(&self, i: usize) -> u64 { self.fact[i] }

    /// $i!^{-1} \\bmod p$ を返す。
    pub fn factorial_recip(&self, i: usize) -> u64 { self.fact_recip[i] }

    /// $i^{-1} \\bmod p$ を返す。
    pub fn recip(&self, i: usize) -> u64 {
        self.fact_recip[i] * self.fact[i - 1] % self.p
    }

    /// $i!/(i-j)! \\bmod p$ を返す。
    pub fn perm(&self, i: usize, j: usize) -> u64 {
        if i < j {
            0
        } else {
            self.fact[i] * self.fact_recip[i - j] % self.p
        }
    }

    /// $i!/(j!\\cdot (i-j)!) \\bmod p$ を返す。
    pub fn binom(&self, i: usize, j: usize) -> u64 {
        if i < j {
            0
        } else {
            self.perm(i, j) * self.fact_recip[j] % self.p
        }
    }
}
