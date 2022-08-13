//! 素数の数え上げ。

use std::ops::RangeFrom;

use bisect::Bisect;
use linear_sieve::LinearSieve;

/// 素数の数え上げ。
///
/// $n$ 以下の素数の個数 $\\pi(n)$ を返す。
///
/// # Complexity
/// $O(\\sqrt{n})$ space, $O(n^{2/3} / \\log(n)^{1/3})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::prime_pi;
///
/// assert_eq!(prime_pi(10), 4);
/// assert_eq!(prime_pi(100), 25);
/// assert_eq!(prime_pi(1000), 168);
/// assert_eq!(prime_pi(10000), 1229);
/// assert_eq!(prime_pi(100_000_000_000), 4118054813);
/// ```
///
/// # References
/// - <https://rsk0315.github.io/slides/prime-counting.pdf>
///
/// # See also
/// - <https://rsk0315.hatenablog.com/entry/2021/05/18/015511>
/// - <https://judge.yosupo.jp/submission/7976>
/// - <https://math314.hateblo.jp/entry/2016/06/05/004332>
/// - <https://projecteuler.net/thread=10;page=5#111677>
pub fn prime_pi(n: usize) -> usize {
    let n_2 = (1_usize..).bisect(|i| i.pow(2) <= n) - 1;
    let n_6 = (1_usize..).bisect(|i| i.pow(6) <= n) - 1;

    let lg = 1.max(n.next_power_of_two().trailing_zeros() as usize);
    let nlg_3 = (1_usize..).bisect(|i| i.pow(3) <= n * lg) - 1;
    let n_lg2_3 = (1_usize..).bisect(|i| i.pow(3) * lg.pow(2) <= n) - 1;

    let mut h = vec![0];
    h.extend((1..=n_2).map(|i| n / i));
    h.extend((1..n / n_2).rev());
    let ns = h.clone();
    for hi in &mut h[1..] {
        *hi -= 1;
    }

    let primes: Vec<_> = LinearSieve::new(n_2).primes().collect();

    let mut pi = 0;
    while pi < primes.len() && primes[pi] <= n_6 {
        let p = primes[pi];
        let pp = p * p;
        for (&n_i, i) in ns[1..].iter().take_while(|&&n_i| n_i >= pp).zip(1..) {
            let ip = i * p;
            h[i] -= h[if ip <= n_2 { ip } else { ns.len() - n_i / p }] - pi;
        }
        pi += 1;
    }

    let thresh = if nlg_3 <= n_2 { nlg_3 } else { ns.len() - n / nlg_3 };
    let mut rsq = FenwickTree::new(ns.len() - thresh);
    while pi < primes.len() && primes[pi] <= n_lg2_3 {
        let p = primes[pi];
        let pp = p * p;
        for (&n_i, i) in
            ns[1..].iter().take_while(|&&n_i| n_i >= pp).zip(1..=thresh)
        {
            let ip = i * p;
            let index = if ip <= n_2 { ip } else { ns.len() - n_i / p };

            let mut sum: usize = h[index];
            if index > thresh {
                sum -= rsq.sum(index - thresh..);
            }
            h[i] -= sum - pi;
        }

        let mut st = vec![(p, pi)];
        while let Some((cur, i)) = st.pop() {
            for (cur, j) in primes[i..]
                .iter()
                .map(|&p_j| cur * p_j)
                .take_while(|&cur| cur < n / nlg_3)
                .zip(i..)
            {
                let index = if cur <= n_2 { ns.len() - cur } else { n / cur };
                if index > thresh {
                    rsq.add(index - thresh, 1);
                }
                st.push((cur, j));
            }
        }

        pi += 1;
    }

    let rsq = rsq.into_suffix_sum();
    for i in 0..rsq.len() {
        h[i + thresh] -= rsq[i];
    }

    while pi < primes.len() && primes[pi] <= n_2 {
        let p = primes[pi];
        let pp = p * p;
        for (&n_i, i) in ns[1..].iter().take_while(|&&n_i| n_i >= pp).zip(1..) {
            let ip = i * p;
            h[i] -= h[if ip <= n_2 { ip } else { ns.len() - n_i / p }] - pi;
        }
        pi += 1;
    }

    h[1]
}

struct FenwickTree(Vec<usize>);

impl FenwickTree {
    pub fn new(n: usize) -> Self { Self(vec![0; n]) }
    pub fn sum(&self, rf: RangeFrom<usize>) -> usize {
        let mut i = rf.start;
        let mut res = 0;
        while i < self.0.len() {
            res += self.0[i];
            i += i & i.wrapping_neg();
        }
        res
    }
    pub fn add(&mut self, mut i: usize, d: usize) {
        while i > 0 {
            self.0[i] += d;
            i -= i & i.wrapping_neg();
        }
    }
    pub fn into_suffix_sum(self) -> Vec<usize> {
        let mut res = self.0;
        for i in (1..res.len()).rev() {
            let j = i + (i & i.wrapping_neg());
            if j < res.len() {
                res[i] += res[j];
            }
        }
        res
    }
}
