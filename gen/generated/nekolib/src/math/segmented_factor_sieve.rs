use super::sqrt;
use std::fmt::Debug;
use std::ops::RangeInclusive;

use sqrt::Sqrt;

#[derive(Debug)]
pub struct SegmentedFactorSieve {
    ofs: usize,
    small_p: Vec<usize>,
    large_p: Vec<Vec<usize>>,
    large_prod: Vec<usize>,
}

impl SegmentedFactorSieve {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        let &l = range.start();
        let r = *range.end();
        let r0 = r.sqrt();
        let mut large = vec![true; r - l + 1];
        let mut small_p: Vec<_> = (0..=r0).collect();
        let mut large_p = vec![vec![]; r - l + 1];
        let mut large_prod: Vec<_> = (l..=r).collect();
        for i in (2..).take_while(|&i| i * i <= r) {
            if small_p[i] < i {
                continue;
            }
            for j in i..=r0 / i {
                if small_p[i * j] == i * j {
                    small_p[i * j] = i;
                }
            }
            for j in (1 + (l - 1) / i..=r / i).filter(|&j| j > 1) {
                let m = i * j - l;
                large[m] = false;
                while large_prod[m] > r0 && large_prod[m] % i == 0 {
                    large_p[m].push(i);
                    large_prod[m] /= i;
                }
            }
        }
        Self { ofs: l, small_p, large_p, large_prod }
    }

    pub fn factors_dup(&self, n: usize) -> impl Iterator<Item = usize> {
        let mut res = self.large_p[n - self.ofs].clone();
        let mut m = self.large_prod[n - self.ofs];
        if m < self.small_p.len() {
            while m > 1 {
                res.push(self.small_p[m]);
                m /= self.small_p[m];
            }
        } else {
            res.push(m);
        }
        res.into_iter()
    }

    pub fn factors(&self, n: usize) -> impl Iterator<Item = (usize, u32)> {
        let dup: Vec<_> = self.factors_dup(n).collect();
        if dup.is_empty() {
            return vec![].into_iter();
        }
        let mut res = vec![(dup[0], 1)];
        for &p in &dup[1..] {
            if res.last().unwrap().0 == p {
                res.last_mut().unwrap().1 += 1;
            } else {
                res.push((p, 1));
            }
        }
        res.into_iter()
    }

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
}
