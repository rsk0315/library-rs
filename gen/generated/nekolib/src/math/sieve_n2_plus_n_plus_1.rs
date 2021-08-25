//! $n^2+n+1$ 型素数の篩。

use std::fmt::Debug;

/// $n^2+n+1$ 型素数の篩。
///
/// # Idea
/// `todo!()`
///
/// # References
/// - <https://inamori.hateblo.jp/entry/20110930/p1>
#[derive(Clone, Debug)]
pub struct SieveN2PlusNPlus1 {
    a: Vec<usize>,
    f: Vec<Vec<(usize, u32)>>,
}

impl SieveN2PlusNPlus1 {
    /// 初期化する。
    pub fn new(n: usize) -> Self {
        let mut a: Vec<_> = (0..=n).map(|k| k * (k + 1) + 1).collect();
        let mut f = vec![vec![]; n + 1];
        f[1] = vec![(3, 1)];
        for j in (4..=n).step_by(3) {
            let (d, e) = Self::div_pow(a[j], 3);
            f[j].push((3, e));
            a[j] = d;
        }
        for i in 2..=n {
            let p = a[i];
            if p == 1 {
                continue;
            }
            if p == i * (i + 1) + 1 {
                f[i].push((p, 1));
            }
            let init1 = if p == i * (i + 1) + 1 { p + i } else { i };
            let init2 = (n + p - 1) / p * p;
            for j in (init1..=n).step_by(p).chain((init2..=n).step_by(p)) {
                let (d, e) = Self::div_pow(a[j], p);
                if e > 0 {
                    f[j].push((p, e));
                    a[j] = d;
                }
            }
        }
        Self { a, f }
    }

    fn div_pow(mut a: usize, p: usize) -> (usize, u32) {
        let mut e = 0;
        while a % p == 0 {
            a /= p;
            e += 1;
        }
        (a, e)
    }

    /// $n^2+n+1$ の形の素数を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveN2PlusNPlus1;
    ///
    /// let ss = SieveN2PlusNPlus1::new(10);
    /// let primes: Vec<_> = ss.primes().collect();
    /// assert_eq!(primes, [3, 7, 13, 31, 43, 73]);
    /// ```
    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        (1..self.a.len())
            .filter(move |&i| self.a[i] == i * (i + 1) + 1)
            .map(|i| i * (i + 1) + 1)
    }

    /// $n^2+n+1$ が素数のとき真を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveN2PlusNPlus1;
    ///
    /// let ss = SieveN2PlusNPlus1::new(10);
    /// assert!(ss.is_prime(3));  // 13 is prime
    /// assert!(!ss.is_prime(4));  // 21 = 3 * 7
    /// ```
    pub fn is_prime(&self, n: usize) -> bool {
        n > 0 && self.a[n] == n * (n + 1) + 1
    }

    /// $n^2+n+1$ を素因数分解する。
    ///
    /// 底の昇順とは限らないので注意。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveN2PlusNPlus1;
    ///
    /// let ss = SieveN2PlusNPlus1::new(20);
    /// assert_eq!(ss.factors(0).next(), None);
    /// assert_eq!(ss.factors(7).collect::<Vec<_>>(), [(3, 1), (19, 1)]);
    /// assert_eq!(ss.factors(18).collect::<Vec<_>>(), [(7, 3)]);
    /// ```
    pub fn factors(&self, n: usize) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.f[n].iter().cloned()
    }

    /// $n^2+1$ を素因数を列挙する。重複あり。
    ///
    /// 底の昇順とは限らないので注意。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveN2PlusNPlus1;
    ///
    /// let ss = SieveN2PlusNPlus1::new(20);
    /// assert_eq!(ss.factors_dup(0).next(), None);
    /// assert_eq!(ss.factors_dup(6).collect::<Vec<_>>(), [43]);
    /// assert_eq!(ss.factors_dup(18).collect::<Vec<_>>(), [7, 7, 7]);
    /// ```
    pub fn factors_dup(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
        self.factors(n).flat_map(|(p, e)| std::iter::repeat(p).take(e as usize))
    }
}

#[test]
fn test() {
    use linear_sieve::LinearSieve;

    let n = 3000;
    let ls = LinearSieve::new(n * (n + 1) + 1);
    let ss = SieveN2PlusNPlus1::new(n);
    for i in 0..=n {
        assert_eq!(ls.is_prime(i * (i + 1) + 1), ss.is_prime(i));

        {
            // factors
            let expected: Vec<_> = ls.factors_dup(i * (i + 1) + 1).collect();
            let mut actual: Vec<_> = ss.factors_dup(i).collect();
            actual.sort_unstable();
            assert_eq!(actual, expected);
        }
    }
}
