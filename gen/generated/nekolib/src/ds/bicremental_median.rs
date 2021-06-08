//! 中央値の管理。

use std::collections::BTreeMap;
use std::fmt::Debug;

/// 中央値の管理。
///
/// 多重集合への要素の追加と削除を行いつつ、中央値を管理する。
///
/// # Naming
/// incremental と decremental の双方向の処理を行うので、bidirectional
/// の気持ちで bicremental とした。記憶が正しければえびちゃんの造語なので、
/// もっとよい名前があれば変えたい。dynamic は意味が曖昧なのできらい。
///
/// # Notes
/// 集合に $k$ 個追加・削除する操作をサポートできないか？と思ったが、
/// これだと計算量の保証ができない。$\\{\\{1, 2, \\dots, n\\}\\}$ に対して
/// $0$ を $n$ 個追加する操作と、$0$ を $n$ 個削除する操作を繰り返すことで、
/// 簡単に worst $\\Omega(n)$ 時間になってしまう。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BicrementalMedian<T: Ord + Clone> {
    lower_len: usize,
    upper_len: usize,
    lower: BTreeMap<T, usize>,
    upper: BTreeMap<T, usize>,
}

impl<T: Ord + Clone> BicrementalMedian<T> {
    pub fn new() -> Self {
        Self {
            lower_len: 0,
            upper_len: 0,
            lower: BTreeMap::new(),
            upper: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, x: T) {
        if self.lower_len == 0 {
            self.lower.insert(x, 1);
            self.lower_len += 1;
        } else if self.lower_len == self.upper_len {
            // [LLL] X [RRR]
            if &x <= self.upper.iter().next().unwrap().0 {
                // [LLXL] [RRR]
                *self.lower.entry(x).or_insert(0) += 1;
            } else {
                // [LLLR] [RRX]
                self.rotate_to_lower();
                *self.upper.entry(x).or_insert(0) += 1;
            }
            self.lower_len += 1;
        } else {
            // [LLL] X [RR]
            if self.lower.iter().next_back().unwrap().0 < &x {
                // [LLL] [RXR]
                *self.upper.entry(x).or_insert(0) += 1;
            } else {
                // [XLL] [LRR]
                self.rotate_to_upper();
                *self.lower.entry(x).or_insert(0) += 1;
            }
            self.upper_len += 1;
        }
    }
    pub fn remove(&mut self, x: &T) -> bool {
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
    pub fn median(&self) -> Option<&T> {
        if self.lower_len == 0 {
            None
        } else {
            Some(self.lower.iter().next_back().unwrap().0)
        }
    }
}

impl<T: Ord + Clone> BicrementalMedian<T> {
    fn rotate_to_lower(&mut self) {
        let (x, k) =
            self.upper.iter().next().map(|(x, &k)| (x.clone(), k)).unwrap();
        if k == 1 {
            self.upper.remove(&x);
        } else {
            *self.upper.get_mut(&x).unwrap() -= 1;
        }
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
        *self.upper.entry(x).or_insert(0) += 1;
    }
    fn remove_from_lower(&mut self, x: &T, rotate: bool) {
        if self.lower[x] == 1 {
            self.lower.remove(x);
        } else {
            *self.lower.get_mut(x).unwrap() -= 1;
        }
        if rotate {
            self.rotate_to_lower();
            self.upper_len -= 1;
        } else {
            self.lower_len -= 1;
        }
    }
    fn remove_from_upper(&mut self, x: &T, rotate: bool) {
        if self.upper[x] == 1 {
            self.upper.remove(x);
        } else {
            *self.upper.get_mut(x).unwrap() -= 1;
        }
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
    let n = 32768;
    let mut f =
        std::iter::successors(Some(296), |&x| Some((x * 258 + 185) % 397))
            .map(|x| x & 15);
    let mut bucket = vec![0; 8];
    let mut bm = BicrementalMedian::<usize>::new();
    for _ in 0..n {
        let x = f.next().unwrap();
        let (remove, x) = (x & 8 != 0, x & 7);
        if remove && bucket[x] > 0 {
            bucket[x] -= 1;
            bm.remove(&x);
        } else {
            bucket[x] += 1;
            bm.insert(x);
        }
        let mut naive = vec![];
        for i in 0..8 {
            naive.extend(std::iter::repeat(i).take(bucket[i]));
        }
        assert_eq!(bm.median(), naive.get(naive.len().wrapping_sub(1) / 2));
    }
}
