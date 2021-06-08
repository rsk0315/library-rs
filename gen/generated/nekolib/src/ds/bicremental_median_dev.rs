//! 中央値と偏差の管理。

use super::super::traits::binop;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::marker::PhantomData;

use binop::CommutativeGroup;

/// 中央値と偏差の管理。
///
/// 多重集合 $S$ への要素の追加と削除を行いつつ、中央値と偏差を管理する。
/// ここで、偏差は中央値との絶対値の総和とする。
/// $S$ のうち小さい方から $\\lceil|S|/2\\rceil$ 個取り出したものを $L$、
/// 残りの $\\lfloor|S|/2\\rfloor$ 個取り出したものを $R$ とする。
/// 中央値を $L$ の最大値として定義し、$a\_{\\text{med}}$ と書く。
/// このとき、偏差 $\\sigma$ は次のように書ける。
/// $$ \\sigma = \\sum\_{a\\in L} (a\_{\\text{med}}-a) + \\sum\_{a\\in R} (a-a\_{\\text{med}}). $$
/// $\\sum\_{a\\in L} a = \\sigma\_L$、$\\sum\_{a\\in R} a = \\sigma\_R$ とすると、
/// $$ \\begin{aligned}
/// \\sigma &= \\begin{cases}
/// -\\sigma\_L + \\sigma\_R, & \\text{if } |L| = |R|; \\\\
/// -\\sigma\_L + \\sigma\_R + a\_{\\text{med}}, & \\text{if } |L| = |R|+1. \\\\
/// \\end{cases}
/// \\end{aligned} $$
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BicrementalMedianDev<M: CommutativeGroup>
where
    M::Set: Ord + Clone,
{
    lower_sum: M::Set,
    upper_sum: M::Set,
    lower_len: usize,
    upper_len: usize,
    lower: BTreeMap<M::Set, usize>,
    upper: BTreeMap<M::Set, usize>,
    _p: PhantomData<M>,
}

impl<M: CommutativeGroup> BicrementalMedianDev<M>
where
    M::Set: Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            lower_sum: M::id(),
            upper_sum: M::id(),
            lower_len: 0,
            upper_len: 0,
            lower: BTreeMap::new(),
            upper: BTreeMap::new(),
            _p: PhantomData,
        }
    }
    pub fn insert(&mut self, x: M::Set) {
        if self.lower_len == 0 {
            self.lower_sum = M::op(self.lower_sum.clone(), x.clone());
            self.lower.insert(x, 1);
            self.lower_len += 1;
        } else if self.lower_len == self.upper_len {
            // [LLL] X [RRR]
            if &x <= self.upper.iter().next().unwrap().0 {
                // [LLXL] [RRR]
                self.lower_sum = M::op(self.lower_sum.clone(), x.clone());
                *self.lower.entry(x).or_insert(0) += 1;
            } else {
                // [LLLR] [RRX]
                self.upper_sum = M::op(self.upper_sum.clone(), x.clone());
                self.rotate_to_lower();
                *self.upper.entry(x).or_insert(0) += 1;
            }
            self.lower_len += 1;
        } else {
            // [LLL] X [RR]
            if self.lower.iter().next_back().unwrap().0 < &x {
                // [LLL] [RXR]
                self.upper_sum = M::op(self.upper_sum.clone(), x.clone());
                *self.upper.entry(x).or_insert(0) += 1;
            } else {
                // [XLL] [LRR]
                self.lower_sum = M::op(self.lower_sum.clone(), x.clone());
                self.rotate_to_upper();
                *self.lower.entry(x).or_insert(0) += 1;
            }
            self.upper_len += 1;
        }
    }
    pub fn remove(&mut self, x: M::Set) -> bool {
        if self.lower_len == 0 {
            false
        } else if self.lower_len == self.upper_len {
            // [LLL] [RRR]
            if self.upper.contains_key(&x) {
                // [LLL] [RR]
                self.remove_from_upper(x, false);
                return true;
            }
            if self.lower.contains_key(&x) {
                // [LLR] [RR]
                self.remove_from_lower(x, true);
                return true;
            }
            false
        } else {
            // [LLL] [RR]
            if self.lower.contains_key(&x) {
                // [LL] [RR]
                self.remove_from_lower(x, false);
                return true;
            }
            if self.upper.contains_key(&x) {
                // [LL] [LR]
                self.remove_from_upper(x, true);
                return true;
            }
            false
        }
    }
    pub fn median(&self) -> Option<&M::Set> {
        if self.lower_len == 0 {
            None
        } else {
            Some(self.lower.iter().next_back().unwrap().0)
        }
    }
    pub fn median_dev(&self) -> M::Set {
        if self.lower_len == 0 {
            M::id()
        } else {
            let diff =
                M::op(self.upper_sum.clone(), M::recip(self.lower_sum.clone()));
            if self.lower_len == self.upper_len {
                diff
            } else {
                M::op(diff, self.median().unwrap().clone())
            }
        }
    }
}

impl<M: CommutativeGroup> BicrementalMedianDev<M>
where
    M::Set: Ord + Clone,
{
    fn rotate_to_lower(&mut self) {
        let (x, k) =
            self.upper.iter().next().map(|(x, &k)| (x.clone(), k)).unwrap();
        if k == 1 {
            self.upper.remove(&x);
        } else {
            *self.upper.get_mut(&x).unwrap() -= 1;
        }
        self.upper_sum = M::op(self.upper_sum.clone(), M::recip(x.clone()));
        self.lower_sum = M::op(self.lower_sum.clone(), x.clone());
        *self.lower.entry(x).or_insert(0) += 1;
    }
    fn rotate_to_upper(&mut self) {
        let (x, k) = self
            .lower
            .iter()
            .next_back()
            .map(|(x, &k)| (x.clone(), k))
            .unwrap();
        if k == 1 {
            self.lower.remove(&x);
        } else {
            *self.lower.get_mut(&x).unwrap() -= 1;
        }
        self.lower_sum = M::op(self.lower_sum.clone(), M::recip(x.clone()));
        self.upper_sum = M::op(self.upper_sum.clone(), x.clone());
        *self.upper.entry(x).or_insert(0) += 1;
    }
    fn remove_from_lower(&mut self, x: M::Set, rotate: bool) {
        if self.lower[&x] == 1 {
            self.lower.remove(&x);
        } else {
            *self.lower.get_mut(&x).unwrap() -= 1;
        }
        self.lower_sum = M::op(self.lower_sum.clone(), M::recip(x));
        if rotate {
            self.rotate_to_lower();
            self.upper_len -= 1;
        } else {
            self.lower_len -= 1;
        }
    }
    fn remove_from_upper(&mut self, x: M::Set, rotate: bool) {
        if self.upper[&x] == 1 {
            self.upper.remove(&x);
        } else {
            *self.upper.get_mut(&x).unwrap() -= 1;
        }
        self.upper_sum = M::op(self.upper_sum.clone(), M::recip(x));
        if rotate {
            self.rotate_to_upper();
            self.lower_len -= 1;
        } else {
            self.upper_len -= 1;
        }
    }
}

#[test]
fn test_simple() {
    use op_add::OpAdd;

    let n = 32768;
    let mut f =
        std::iter::successors(Some(296), |&x| Some((x * 258 + 185) % 397))
            .map(|x| x & 15);
    let mut bucket = vec![0; 8];
    let mut bm = BicrementalMedianDev::<OpAdd<i32>>::new();
    for _ in 0..n {
        let x = f.next().unwrap();
        let (remove, x) = (x & 8 != 0, x & 7);
        if remove && bucket[x as usize] > 0 {
            bucket[x as usize] -= 1;
            bm.remove(x);
        } else {
            bucket[x as usize] += 1;
            bm.insert(x);
        }
        let mut naive = vec![];
        for i in 0..8 {
            naive.extend(std::iter::repeat(i as i32).take(bucket[i]));
        }
        assert_eq!(bm.median(), naive.get(naive.len().wrapping_sub(1) / 2));
        let &median = bm.median().unwrap_or(&0);
        eprintln!("{:?}", naive);
        eprintln!("{:?}", bm);
        let dev: i32 = naive.iter().map(|&x| (x - median).abs()).sum();
        assert_eq!(bm.median_dev(), dev);
    }
}
