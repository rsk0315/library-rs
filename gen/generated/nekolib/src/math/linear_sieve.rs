//! 線形篩。

use super::gcd_recip;

use gcd_recip::GcdRecip;

/// 線形篩。
///
/// # Idea
/// 各整数の最小素因数を求めつつ、素数のリストを作成していく。
///
/// $\\gdef\\lpf#1{\\operatorname{lpf}(#1)}$
/// $i$ の最小素因数を $\\mathrm{lpf}(i)$ と書く。$j \\le \\lpf{i}$
/// なる各素数 $j$ に対して、$\\lpf{i\\times j} = j$ とわかる。
/// 素因数分解の一意性から、各整数の最小素因数の更新は一回ずつしか行われず、線形時間で構築できる。
///
/// また、$\\lpf{i}$ が $\\lpf{i / {\\lpf{i}}}$
/// と等しいかで場合分けしながら DP することで、各 $i$ が $\\lpf{i}$
/// で何回割り切れるかも求められる。
/// なお、これは DP せず各 $i$ に対して愚直に計算しても $O(n)$ になることが示せるらしい。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(n)$|
/// |`is_prime`|$\\Theta(1)$|
/// |`least_factor`|$\\Theta(1)$|
/// |`factors_dup`|$\\Theta(1)$ delay|
/// |`factors`|$\\Theta(1)$ delay|
/// |`euler_phi`|$\\Theta(\\omega(n))$|
/// |`euler_phi_star`|$O(\\omega(n)\\log(n))$|
/// |`divisors`|$O(\\sigma(n))$|
/// |`divisors_count`|$O(\\omega(n))$|
/// |`divisors_sum`|$O(\\omega(n))$|
/// |`primes`|$\\Theta(1)$ delay|
/// |`recips`|$O(n)$|
///
/// $n$ の素因数の個数を $\\omega(n)$ とすると、以下の式が成り立つらしい。
/// $$ \\sum\_{i\\le n} \\omega(i) = n\\ln(\\ln(n)) + O(n). $$
///
/// また、$n$ 以下の素数の個数を $\\pi(n)$ とすると、以下の式が成り立つ（素数定理）。
/// $$ \\pi(n) = \\frac{n}{\\ln(n)} + O{\\left(\\frac{n}{\\ln(n)\^2}\\right)}. $$
///
/// # Examples
/// ```
/// use nekolib::math::LinearSieve;
///
/// let sieve = LinearSieve::new(60);
/// assert!(!sieve.is_prime(1));
/// assert!(sieve.is_prime(2));
/// assert!(sieve.is_prime(23));
/// assert!(!sieve.is_prime(24));
///
/// assert_eq!(sieve.least_factor(1), None);
/// assert_eq!(sieve.least_factor(3), Some(3));
/// assert_eq!(sieve.least_factor(24), Some(2));
///
/// assert_eq!(sieve.factors_dup(1).next(), None);
/// assert_eq!(sieve.factors_dup(60).collect::<Vec<_>>(), vec![2, 2, 3, 5]);
///
/// assert_eq!(
///     sieve.primes().take(10).collect::<Vec<_>>(),
///     vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
/// );
/// ```
///
/// # References
/// - <https://cp-algorithms.com/algebra/prime-sieve-linear.html>
/// - <https://twitter.com/hidesugar2/status/1431243186362458114>
/// - <https://maspypy.com/%E7%B4%A0%E6%95%B0%E3%81%AB%E9%96%A2%E3%81%99%E3%82%8B%E4%B8%8A%E3%81%8B%E3%82%89%E3%81%AE%E8%A9%95%E4%BE%A1%EF%BC%88%E5%88%9D%E7%AD%89%E7%9A%84%E3%81%AA%E8%A8%BC%E6%98%8E%EF%BC%89>
pub struct LinearSieve {
    lpf: Vec<usize>,
    lpf_e: Vec<(usize, u32)>,
    pr: Vec<usize>,
}

impl LinearSieve {
    /// $n$ 以下の自然数に対する篩を用意する。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// ```
    pub fn new(n: usize) -> Self {
        let mut lpf = vec![1; n + 1];
        let mut pr = vec![];
        for i in 2..=n {
            if lpf[i] == 1 {
                lpf[i] = i;
                pr.push(i);
            }
            let lpf_i = lpf[i];
            for &j in pr.iter().take_while(|&&j| j <= lpf_i && i * j <= n) {
                lpf[i * j] = j;
            }
        }
        let mut lpf_e = vec![(1, 0); n + 1];
        for i in 2..=n {
            let p = lpf[i];
            let j = i / p;
            lpf_e[i] = if lpf[j] == p {
                (lpf_e[j].0 * p, lpf_e[j].1 + 1)
            } else {
                (lpf[i], 1)
            };
        }
        Self { lpf, lpf_e, pr }
    }

    /// $n$ が素数であれば `true` を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert!(!sieve.is_prime(1));
    /// assert!(sieve.is_prime(2));
    /// assert!(sieve.is_prime(23));
    /// assert!(!sieve.is_prime(24));
    /// ```
    pub fn is_prime(&self, n: usize) -> bool { n >= 2 && self.lpf[n] == n }

    /// $n$ の最小素因数を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.least_factor(1), None);
    /// assert_eq!(sieve.least_factor(3), Some(3));
    /// assert_eq!(sieve.least_factor(24), Some(2));
    /// ```
    pub fn least_factor(&self, n: usize) -> Option<usize> {
        if n < 2 { None } else { Some(self.lpf[n]) }
    }

    /// $n$ の素因数を列挙する。重複あり。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.factors_dup(1).next(), None);
    /// assert_eq!(sieve.factors_dup(60).collect::<Vec<_>>(), vec![2, 2, 3, 5]);
    /// ```
    pub fn factors_dup(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf[n]))
            .take_while(|&n| n > 1)
            .map(move |n| self.lpf[n])
    }

    /// $n$ を素因数分解する。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.factors(1).next(), None);
    /// assert_eq!(
    ///     sieve.factors(60).collect::<Vec<_>>(),
    ///     vec![(2, 2), (3, 1), (5, 1)]
    /// );
    /// ```
    pub fn factors(&self, n: usize) -> impl Iterator<Item = (usize, u32)> + '_ {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf_e[n].0))
            .take_while(|&n| n > 1)
            .map(move |n| (self.lpf[n], self.lpf_e[n].1))
    }

    /// $\\phi(n)$ を求める。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.euler_phi(1), 1);
    /// assert_eq!(sieve.euler_phi(35), 24);
    /// assert_eq!(sieve.euler_phi(60), 16);
    /// ```
    pub fn euler_phi(&self, n: usize) -> usize {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf_e[n].0))
            .take_while(|&n| n > 1)
            .map(|n| self.lpf_e[n].0 / self.lpf[n] * (self.lpf[n] - 1))
            .product()
    }

    /// $\\phi^\\star(n)$ を求める。
    ///
    /// $n$ に $\\phi$ を繰り返し適用して $1$ にするために必要な最小回数である。
    /// $n\\gt 1$ に対して $\\phi(\\phi(n)) \\le n/2$ なので、
    /// $\\phi^\\star(n) = O(\\log(n))$ である。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.euler_phi_star(1), 0);
    /// assert_eq!(sieve.euler_phi_star(35), 5);
    /// assert_eq!(sieve.euler_phi_star(60), 5);
    ///
    /// assert_eq!(sieve.euler_phi(35), 24);
    /// assert_eq!(sieve.euler_phi(24), 8);
    /// assert_eq!(sieve.euler_phi(8), 4);
    /// assert_eq!(sieve.euler_phi(4), 2);
    /// assert_eq!(sieve.euler_phi(2), 1);
    ///
    /// assert_eq!(sieve.euler_phi(60), 16);
    /// assert_eq!(sieve.euler_phi(16), 8);
    /// ```
    pub fn euler_phi_star(&self, n: usize) -> usize {
        match n {
            0..=2 => n / 2,
            _ => 1 + self.euler_phi_star(self.euler_phi(n)),
        }
    }

    /// $n$ の約数を列挙する。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.divisors(1).next(), Some(1));
    /// assert_eq!(sieve.divisors(1).nth(1), None);
    /// assert_eq!(
    ///     sieve.divisors(60).collect::<Vec<_>>(),
    ///     vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
    /// );
    /// ```
    pub fn divisors(
        &self,
        n: usize,
    ) -> impl Iterator<Item = usize> + DoubleEndedIterator {
        let mut res = vec![1];
        for (p, e) in self.factors(n) {
            let mut tmp = vec![];
            let mut pp = 1;
            for _ in 1..=e {
                pp *= p;
                tmp.extend(res.iter().map(|&x| x * pp));
            }
            res.extend(tmp);
        }
        res.sort_unstable();
        res.into_iter()
    }

    /// $n$ の約数の個数を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.divisors_count(1), 1);
    /// assert_eq!(sieve.divisors_count(60), sieve.divisors(60).count());
    /// ```
    pub fn divisors_count(&self, n: usize) -> usize {
        self.factors(n).map(|(_, e)| e as usize + 1).product()
    }

    /// $n$ の約数の総和を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.divisors_sum(1), 1);
    /// assert_eq!(sieve.divisors_sum(8), 15);
    /// assert_eq!(sieve.divisors_sum(60), 168);
    /// ```
    pub fn divisors_sum(&self, n: usize) -> usize {
        std::iter::successors(Some(n), move |&n| Some(n / self.lpf_e[n].0))
            .take_while(|&n| n > 1)
            .map(|n| (self.lpf_e[n].0 * self.lpf[n] - 1) / (self.lpf[n] - 1))
            .product()
    }

    /// 素数を列挙する。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(
    ///     sieve.primes().take(10).collect::<Vec<_>>(),
    ///     vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    /// );
    /// ```
    pub fn primes(
        &self,
    ) -> impl Iterator<Item = usize> + DoubleEndedIterator + '_ {
        self.pr.iter().copied()
    }

    /// 法 $m$ での逆元を返す。
    ///
    /// $\\gdef\\recip#1#2{#1^{-1}\_{(#2)}}$
    /// $i^{-1}\\bmod j$ を $\\recip{i}{j}$ と書く。
    /// 次で定められる $a = (a\_0, a\_1, \\dots, a\_n)$ を返す。
    /// $$
    /// a\_i = \\begin{cases}
    /// \\recip{i}{m}, & \\text{if }\\recip{i}{m}\\text{ exists}; \\\\
    /// 0, & \\text{otherwise}.
    /// \\end{cases}
    /// $$
    /// Note: $\\recip{i}{m}\\ne 0$。
    ///
    /// # Idea
    ///
    /// $$
    /// \\recip{i}{m} \\equiv \\recip{\\lpf{i}}{m}\\cdot\\recip{(i/{\\lpf{i}})}{m}\\pmod{m}
    /// $$
    /// に基づく。素数は $O(n/\\log(n))$ 個しかないため、互除法で愚直に求めても全体では
    /// $O(n)$ 時間となる。
    ///
    /// # See also
    /// <https://37zigen.com/linear-sieve/#i-4>
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.recips(0, 1), [0]);
    /// assert_eq!(sieve.recips(4, 5), [0, 1, 3, 2, 4]);
    /// assert_eq!(sieve.recips(9, 10), [0, 1, 0, 7, 0, 0, 0, 3, 0, 9]);
    /// ```
    pub fn recips(&self, n: usize, m: usize) -> Vec<usize> {
        assert!(m > 0);
        if n == 0 {
            return vec![0];
        }

        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        for i in 2..=n {
            let lpf_i = self.lpf[i];
            if lpf_i == i {
                if let (1, r) = i.gcd_recip(m) {
                    dp[i] = r;
                }
            } else {
                dp[i] = dp[lpf_i] * dp[i / lpf_i] % m;
            }
        }
        dp
    }

    /// 最小素因数を用いて DP を行う。
    ///
    /// 関数 $f$ であって、任意の $i\\gt 1$ に対して
    /// $$
    /// f(i) = \\begin{cases}
    /// g\_{=}(f(i/j), \\lpf{i}), & \\text{if }\\lpf{i} = \\lpf{i/j}; \\\\
    /// g\_{\\gt}(f(i/j), \\lpf{i}), & \\text{if }\\lpf{i} \\gt \\lpf{i/j} \\\\
    /// \\end{cases}
    /// $$
    /// となるものを計算する。ただし $j = \\lpf{i}$ とする。
    ///
    /// `(zero, one, eq, gt)` はそれぞれ $(f(0), f(1), g\_{=}, g\_{\\gt})$ である。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(10);
    ///
    /// // Moebius mu function
    /// let mu = sieve.dp(0, 1, |&x, _| 0, |&x, _| -x);
    /// assert_eq!(mu, [0, 1, -1, -1, 0, -1, 1, -1, 0, 0, 1]);
    ///
    /// // Euler phi function
    /// let phi = sieve.dp(0, 1, |&x, p| x * p, |&x, p| x * (p - 1));
    /// assert_eq!(phi, [0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]);
    ///
    /// // the number of distinct prime factors
    /// let omega = sieve.dp(0, 0, |&x, _| x, |&x, _| x + 1);
    /// assert_eq!(omega, [0, 0, 1, 1, 1, 1, 2, 1, 1, 1, 2]);
    ///
    /// // the number of prime factors
    /// let cap_omega = sieve.dp(0, 0, |&x, _| x + 1, |&x, _| x + 1);
    /// assert_eq!(cap_omega, [0, 0, 1, 1, 2, 1, 2, 1, 3, 2, 2]);
    ///
    /// // sum of divisors
    /// let eq = |&(prod, sum, pow): &_, p| (prod, sum + pow * p, pow * p);
    /// let gt = |&(prod, sum, _): &_, p| (prod * sum, 1 + p, p);
    /// let sigma: Vec<_> = sieve.dp((1, 1, 1), (1, 1, 1), eq, gt)
    ///     .into_iter().map(|(prod, sum, _)| prod * sum)
    ///     .collect();
    /// assert_eq!(sigma, [1, 1, 3, 4, 7, 6, 12, 8, 15, 13, 18]);
    /// ```
    pub fn dp<T>(
        &self,
        zero: T,
        one: T,
        eq: impl Fn(&T, usize) -> T,
        gt: impl Fn(&T, usize) -> T,
    ) -> Vec<T> {
        let n = self.lpf.len() - 1;

        if n == 0 {
            return vec![zero];
        } else if n == 1 {
            return vec![zero, one];
        }

        let mut res = vec![zero, one];
        res.reserve(n + 1);
        for i in 2..=n {
            let lpf = self.lpf[i];
            let tmp = if lpf == self.lpf[i / lpf] {
                eq(&res[i / lpf], lpf)
            } else {
                gt(&res[i / lpf], lpf)
            };
            res.push(tmp);
        }
        res
    }
}

#[test]
fn test_recips() {
    let m_max = 2000;
    let ls = LinearSieve::new(m_max);
    for m in 2..=m_max {
        let n = m - 1;
        let actual = ls.recips(m_max, m);
        for i in 0..=n {
            let recip = actual[i];
            if recip == 0 {
                assert_ne!(i.gcd_recip(m).0, 1);
            } else {
                assert!(recip < m);
                assert_eq!(i * recip % m, 1);
            }
        }
    }
}
