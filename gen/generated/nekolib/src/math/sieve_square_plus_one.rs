//! $n^2+1$ 型素数の篩。

use std::fmt::Debug;

/// $n^2+1$ 型素数の篩。
///
/// # Idea
/// $n^2+1$ が素因数 $p < n$ を持つとき、以下が成り立つ。
/// $$ n^2+1 &\\equiv 0 \\pmod{p}. $$
/// このとき、各 $k$ に対して以下が成り立つ。
///
/// - $(n+kp)^2+1 \\equiv 0 \\pmod{p}$,
/// - $(kp-n)^2+1 \\equiv 0 \\pmod{p}$.
///
/// # References
/// - <https://inamori.hateblo.jp/entry/20100510/p1>
///
/// # See also
/// - <https://inamori.hateblo.jp/entry/20110930/p1>
///     - $n^2+n+1$ 型素数についての話。
/// - <https://twitter.com/min_25_/status/1418781997892136960>
/// - <https://twitter.com/min_25_/status/1418794801625853952>
///     - $an^2+bn+c$ 型素数についての話。
#[derive(Clone, Debug)]
pub struct SieveSquarePlusOne {
    a: Vec<usize>,
    f: Vec<Vec<(usize, u32)>>,
}

impl SieveSquarePlusOne {
    /// 初期化する。
    pub fn new(n: usize) -> Self {
        let mut a: Vec<_> = (0..=n).map(|k| k * k + 1).collect();
        let mut f = vec![vec![]; n + 1];
        f[1] = vec![(2, 1)];
        for j in (3..=n).step_by(2) {
            let (d, e) = Self::div_pow(a[j], 2);
            f[j].push((2, e));
            a[j] = d;
        }
        for i in 2..=n {
            let p = a[i];
            if p == 1 {
                continue;
            }
            if p == i * i + 1 {
                f[i].push((p, 1));
            }
            let init1 = if p == i * i + 1 { p + i } else { i };
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

    /// $n^2+1$ が素数のとき真を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveSquarePlusOne;
    ///
    /// let ss = SieveSquarePlusOne::new(10);
    /// assert!(ss.is_prime(2));  // 5 is prime
    /// assert!(!ss.is_prime(5));  // 26 = 2 * 13
    /// ```
    pub fn is_prime(&self, n: usize) -> bool { n > 0 && self.a[n] != 1 }

    /// $n^2+1$ を素因数分解する。
    ///
    /// 底の昇順とは限らないので注意。最小の反例は `n = 21`。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveSquarePlusOne;
    ///
    /// let ss = SieveSquarePlusOne::new(10);
    /// assert_eq!(ss.factors(0).next(), None);
    /// assert_eq!(ss.factors(4).collect::<Vec<_>>(), [(17, 1)]);
    /// assert_eq!(ss.factors(7).collect::<Vec<_>>(), [(2, 1), (5, 2)]);
    /// ```
    pub fn factors(&self, n: usize) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.f[n].iter().cloned()
    }

    /// $n^2+1$ を素因数を列挙する。重複あり。
    ///
    /// 底の昇順とは限らないので注意。最小の反例は `n = 21`。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SieveSquarePlusOne;
    ///
    /// let ss = SieveSquarePlusOne::new(10);
    /// assert_eq!(ss.factors_dup(0).next(), None);
    /// assert_eq!(ss.factors_dup(4).collect::<Vec<_>>(), [17]);
    /// assert_eq!(ss.factors_dup(7).collect::<Vec<_>>(), [2, 5, 5]);
    /// ```
    pub fn factors_dup(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
        self.factors(n).flat_map(|(p, e)| std::iter::repeat(p).take(e as usize))
    }
}
