//! 線形篩。

/// 線形篩。
///
/// # Idea
/// 各整数の最小素因数を求めつつ、素数のリストを作成していく。
///
/// $i$ の最小素因数を $\\mathrm{lpf}(i)$ と書く。$j \\le \\mathrm{lpf}(i)$
/// なる各素数 $j$ に対して、$\\mathrm{lpf}(i\\times j) = j$ とわかる。
/// 素因数分解の一意性から、各整数の最小素因数の更新は一回ずつしか行われず、線形時間で構築できる。
///
/// ## See also
/// <https://cp-algorithms.com/algebra/prime-sieve-linear.html>
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(n)$|
/// |`is_prime`|$\\Theta(1)$|
/// |`least_factor`|$\\Theta(1)$|
/// |`factors_dup`|$\\Theta(1)$ delay|
/// |`factors`|$\\Theta(e\_i)$ delay|
/// |`euler_phi`|$\\Theta(\\Omega(n))$|
/// |`primes`|$\\Theta(1)$ delay|
///
/// $n$ の素因数の個数を $\\Omega(n)$ とすると、以下の式が成り立つらしい。
/// $$ \\sum\_{i\\le n} \\Omega(i) = n\\ln(\\ln(n)) + O(n). $$
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
pub struct LinearSieve {
    lpf: Vec<usize>,
    pr: Vec<usize>,
}

impl LinearSieve {
    /// $n$ 以下の自然数に対する篩を用意する。
    ///
    /// # Examples
    ///
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
        Self { lpf, pr }
    }

    /// $n$ が素数であれば `true` を返す。
    ///
    /// # Examples
    ///
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
    ///
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.least_factor(1), None);
    /// assert_eq!(sieve.least_factor(3), Some(3));
    /// assert_eq!(sieve.least_factor(24), Some(2));
    /// ```
    pub fn least_factor(&self, n: usize) -> Option<usize> {
        if n < 2 {
            None
        } else {
            Some(self.lpf[n])
        }
    }

    /// $n$ の素因数を列挙する。重複あり。
    ///
    /// # Examples
    ///
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
    ///
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
        self.factors_dup(n)
            .chain(std::iter::once(1))
            .scan((self.lpf[n], 0), |state, x| {
                if state.0 == x {
                    state.1 += 1;
                    Some(None)
                } else {
                    Some(Some(std::mem::replace(state, (x, 1))))
                }
            })
            .filter_map(std::convert::identity)
    }

    /// $\\phi(n)$ を求める。
    ///
    /// # Examples
    ///
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(sieve.euler_phi(1), 1);
    /// assert_eq!(sieve.euler_phi(35), 24);
    /// assert_eq!(sieve.euler_phi(60), 16);
    /// ```
    pub fn euler_phi(&self, n: usize) -> usize {
        self.factors(n).map(|(p, e)| (p - 1) * p.pow(e as u32 - 1)).product()
    }

    /// $n$ の約数を列挙する。
    ///
    /// # Examples
    ///
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
    pub fn divisors(&self, n: usize) -> impl Iterator<Item = usize> {
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
    ///
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

    /// 素数を列挙する。
    ///
    /// # Examples
    ///
    /// ```
    /// use nekolib::math::LinearSieve;
    ///
    /// let sieve = LinearSieve::new(60);
    /// assert_eq!(
    ///     sieve.primes().take(10).collect::<Vec<_>>(),
    ///     vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    /// );
    /// ```
    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        self.pr.iter().copied()
    }
}
