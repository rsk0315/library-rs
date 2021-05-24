//! $\\sum\_{i=1}^n \\lfloor m/i\\rfloor$ および $\\sum\_{i=1}^n (m\\bmod i)$.

use std::fmt::Debug;
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};

/// $\\sum\_{i=1}^n \\lfloor m/i\\rfloor$ および $\\sum\_{i=1}^n (m\\bmod i)$.
///
/// # Idea
/// $\\lfloor m/\\bullet\\rfloor$ の値は $O(\\sqrt{m})$ 通りである。
/// $i\\in[q\_l, q\_r]$ において $\\lfloor m/i\\rfloor$ の値が共通であるとき、
/// $\\sum\_{i=q\_l}^{q\_r} \\lfloor m/i\\rfloor$ の値は簡単に求められる。
/// また、この範囲で $(m\\bmod i)$ は等差数列を成すことから、
/// $\\sum\_{i=q\_l}^{q\_r} (m\\bmod i)$ も簡単に求められる。
/// 前計算でこれらの累積和を求めておき、差分計算によってクエリ処理を行う。
///
/// # Notes
/// 考察を進めれば $\\sum\_{i=1}^n \\lfloor\\frac{m}{ai+b}\\rfloor$ を求めることも可能？
///
/// # Complexity
/// $O(\\sqrt{m})$ preprocess, $O(1)$ query time.
///
/// # Examples
/// ```
/// use nekolib::math::HarmonicSum;
///
/// let m = 100;
/// let hs = HarmonicSum::new(m);
/// assert_eq!(hs.quot(1..=m), (1..=m).map(|i| m / i).sum());
/// assert_eq!(hs.rem(1..=m), (1..=m).map(|i| m % i).sum());
///
/// let n = 60;
/// assert_eq!(hs.quot(..=n), (1..=n).map(|i| m / i).sum());
/// ```
#[derive(Clone, Debug)]
pub struct HarmonicSum {
    m: i128,
    q: Vec<i128>,
    qsum: Vec<i128>,
    rsum: Vec<i128>,
}

impl HarmonicSum {
    /// 前処理を行う。
    pub fn new(m: i128) -> Self {
        let mut q = vec![0];
        let mut tmp = vec![];
        for i in (1..).take_while(|&i| i * i <= m) {
            q.push(i);
            if i * i < m {
                tmp.push(m / i);
            }
        }
        q.extend(tmp.into_iter().rev());

        let mut qsum = vec![0; q.len()];
        let mut rsum = vec![0; q.len()];
        for i in 1..q.len() {
            let ql = q[i - 1] + 1;
            let qr = q[i];
            let qlen = q[i] - q[i - 1];
            qsum[i] = qsum[i - 1] + m / q[i] * qlen;
            rsum[i] = rsum[i - 1] + (m % ql + m % qr) * qlen / 2;
        }
        Self { m, q, qsum, rsum }
    }
    fn search(&self, n: i128) -> usize {
        let res = if n > self.m {
            self.q.len()
        } else if n * n <= self.m {
            n as usize
        } else {
            self.q.len() - (self.m / n) as usize
        };
        eprintln!("self.m: {}, search({}) = {}", self.m, n, res);
        res
    }
    fn quot_internal(&self, n: i128) -> i128 {
        if n <= 0 {
            return 0;
        }
        let i = self.search(n);
        if i == self.q.len() {
            *self.qsum.last().unwrap()
        } else if self.q[i] == n {
            self.qsum[i]
        } else {
            self.qsum[i - 1] + (n - self.q[i - 1]) * (self.m / n)
        }
    }
    /// $\\sum\_{i=s}^e \\lfloor m/i\\rfloor$ を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::HarmonicSum;
    ///
    /// let m = 100;
    /// let hs = HarmonicSum::new(m);
    /// assert_eq!(hs.quot(1..=m), (1..=m).map(|i| m / i).sum());
    /// ```
    pub fn quot(&self, r: impl RangeBounds<i128>) -> i128 {
        let end = match r.end_bound() {
            Included(&e) => self.quot_internal(e),
            Excluded(&e) => self.quot_internal(e - 1),
            Unbounded => *self.qsum.last().unwrap(),
        };
        let start = match r.start_bound() {
            Included(&s) => self.quot_internal(s - 1),
            Excluded(&s) => self.quot_internal(s),
            Unbounded => 0,
        };
        end - start
    }
    fn rem_internal(&self, n: i128) -> i128 {
        if n <= 0 {
            return 0;
        }
        let i = self.search(n);
        if i == self.q.len() {
            *self.rsum.last().unwrap() + (n - self.m) * self.m
        } else if self.q[i] == n {
            self.rsum[i]
        } else {
            let ql = self.q[i - 1] + 1;
            let len = n - self.q[i - 1];
            self.rsum[i - 1] + (self.m % n + self.m % ql) * len / 2
        }
    }
    /// $\\sum\_{i=s}^e (m\\bmod i)$ を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::HarmonicSum;
    ///
    /// let m = 100;
    /// let hs = HarmonicSum::new(m);
    /// assert_eq!(hs.rem(1..=m), (1..=m).map(|i| m % i).sum());
    /// ```
    pub fn rem(&self, r: impl RangeBounds<i128>) -> i128 {
        let end = match r.end_bound() {
            Included(&e) => self.rem_internal(e),
            Excluded(&e) => self.rem_internal(e - 1),
            Unbounded => *self.rsum.last().unwrap(),
        };
        let start = match r.start_bound() {
            Included(&s) => self.rem_internal(s - 1),
            Excluded(&s) => self.rem_internal(s),
            Unbounded => 0,
        };
        end - start
    }
}

#[test]
fn test_quot() {
    let m = 300;
    let hs = HarmonicSum::new(m);
    for start in 1..=m + 10 {
        let mut sum = 0;
        for end in start..=m + 10 {
            sum += m / end;
            assert_eq!(hs.quot(start..=end), sum);
        }
        let mut sum = 0;
        for end in start..=m + 10 {
            assert_eq!(hs.quot(start..end), sum);
            sum += m / end;
        }
    }
}

#[test]
fn test_rem() {
    let m = 300;
    let hs = HarmonicSum::new(m);
    for start in 1..=m + 10 {
        let mut sum = 0;
        for end in start..=m + 10 {
            sum += m % end;
            assert_eq!(hs.rem(start..=end), sum);
        }
        let mut sum = 0;
        for end in start..=m + 10 {
            assert_eq!(hs.rem(start..end), sum);
            sum += m % end;
        }
    }
}
