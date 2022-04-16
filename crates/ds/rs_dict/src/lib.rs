//! rank/select 辞書。

use std::fmt::Debug;
use std::ops::{Range, RangeBounds};

use buf_range::bounds_within;
use count::Count;
use find_nth::FindNth;

const WORD_SIZE: usize = 64;
const WORD_SIZE_2: usize = WORD_SIZE * WORD_SIZE;

/// rank/select 辞書。
///
/// 要素が `0`/`1` からなる配列で、任意区間の `0`/`1` の個数を数えられる。
///
/// # Idea
/// 要素数 $n$ のビット配列に対して、rank/select のクエリはそれぞれ $n+1$
/// 通りしかないので、それらを $O(n)$ 時間で前計算しておけば、$3n+O(1)$ words で
/// $O(1)$ query time を実現できる[^1]。
///
/// [^1]: $\\mathtt{rank}\_1$ の結果から $\\mathtt{rank}\_0$ の結果を求めることは可能だが、
/// $\\mathtt{select}\_1$ の結果から $\\mathtt{select}\_0$ の結果を求めることはできない。
///
/// しかし、wavelet matrix などに用いる際はこれを 64 本持ったりする必要があることから、
/// 空間を削減できた方がうれしそうなので、$6n/w+O(1)$ words, $O(\\log(w))$ query time
/// の方法を用いた[^2]。
///
/// [^2]: やや複雑なので、もしかすると愚直の方がよい可能性もあるかも？ 実測しましょう。
///
/// rank については、$w$ bits ごとに求めた個数の累積和 ($n/w$ words) を用いる。
/// 端数については word size の popcount を行う[^3]。
///
/// [^3]: $O(\\log(w))$ time の方法は実装していません。`.count_ones()`
/// の実測が遅いようなら考えます。ここを $O(1)$ time と見なしていいかはわかりません。
/// 簡潔データ構造の文脈では、「どうせ表引きできるので...」となっていそうです。
///
/// select については `0` と `1` に対して用意する必要があり、以下では `1` のみ述べるが、
/// `0` についても mutatis mutandis でできる。
/// まず、`1` の $w$ 個おきの出現箇所を求める (at most $n/w$ words)。このうち、幅が
/// $w^2$ 以上であるものを "疎" と呼び、そうでないところを "密" と呼ぶ。
/// 疎である区間は高々 $n/w^2$ 個しかないので、その出現位置を陽に持っても $n/w$
/// words で抑えられる。また、密である区間については、区間幅が $w^2$ 未満なので、
/// クエリごとに二分探索しても $\\log(w)$ time で抑えられる。
///
/// # Complexity
/// $O(n)$ preprocess, $O(n/w)$ space, $O(\\log(w))$ query time.
#[derive(Clone, Debug)]
pub struct RsDict {
    len: usize,
    buf: Vec<u64>,
    rank: Vec<usize>,
    sel0: Vec<SelectPreprocess>,
    sel1: Vec<SelectPreprocess>,
}

#[derive(Clone, Debug)]
enum SelectPreprocess {
    Sparse(Vec<usize>),
    Dense(Range<usize>),
}
use SelectPreprocess::{Dense, Sparse};

impl From<Vec<bool>> for RsDict {
    fn from(buf: Vec<bool>) -> Self {
        let len = buf.len();
        let buf = Self::compress_vec_bool(buf);
        let rank = Self::preprocess_rank(&buf);
        let sel0 = Self::preprocess_select(&buf, len, 0);
        let sel1 = Self::preprocess_select(&buf, len, 1);
        Self { len, buf, rank, sel0, sel1 }
    }
}

impl RsDict {
    fn compress_vec_bool(buf: Vec<bool>) -> Vec<u64> {
        if buf.is_empty() {
            return vec![];
        }
        let n = buf.len();
        let nc = 1 + (n - 1) / WORD_SIZE;
        let mut res = vec![0; nc + 1];
        for i in 0..n {
            if buf[i] {
                res[i / WORD_SIZE] |= 1_u64 << (i % WORD_SIZE);
            }
        }
        res
    }
    fn preprocess_rank(buf: &[u64]) -> Vec<usize> {
        let n = buf.len();
        let mut res = vec![0; n];
        for i in 1..n {
            res[i] = res[i - 1] + buf[i - 1].count_ones() as usize;
        }
        res
    }
    fn preprocess_select(
        buf: &[u64],
        n: usize,
        x: u64,
    ) -> Vec<SelectPreprocess> {
        let mut sel = vec![];
        let mut tmp = vec![];
        let mut last = 0;
        for i in 0..n {
            if buf[i / WORD_SIZE] >> (i % WORD_SIZE) & 1 != x {
                continue;
            }
            if tmp.len() == WORD_SIZE {
                let len = i - last;
                if len < WORD_SIZE_2 {
                    sel.push(Dense(last..i));
                } else {
                    sel.push(Sparse(tmp));
                }
                tmp = vec![];
                last = i;
            }
            tmp.push(i);
        }
        if !tmp.is_empty() {
            sel.push(Sparse(tmp));
        }
        sel
    }
    pub fn rank(&self, end: usize, x: u64) -> usize {
        let il = end / WORD_SIZE;
        let is = end % WORD_SIZE;
        let rank1 = self.rank[il]
            + (self.buf[il] & !(!0_u64 << is)).count_ones() as usize;
        let rank = if x == 0 { end - rank1 } else { rank1 };
        rank
    }
    pub fn select(&self, x: u64, k: usize) -> Option<usize> {
        if self.rank(self.len, x) < k {
            None
        } else if k == 0 {
            Some(0)
        } else {
            Some(self.find_nth_internal(x, k - 1) + 1)
        }
    }
}

impl Count<u64> for RsDict {
    fn count(&self, r: impl RangeBounds<usize>, x: u64) -> usize {
        let Range { start, end } = bounds_within(r, self.len);
        if start > 0 {
            self.rank(end, x) - self.rank(start, x)
        } else {
            self.rank(end, x)
        }
    }
}

impl FindNth<u64> for RsDict {
    fn find_nth(
        &self,
        r: impl RangeBounds<usize>,
        x: u64,
        n: usize,
    ) -> Option<usize> {
        let Range { start, end } = bounds_within(r, self.len);
        if self.count(start..end, x) <= n {
            None
        } else {
            let offset = self.rank(start, x);
            Some(self.find_nth_internal(x, offset + n))
        }
    }
}

impl RsDict {
    fn find_nth_internal(&self, x: u64, n: usize) -> usize {
        if self.rank(self.len, x) < n {
            panic!("the number of {}s is less than {}", x, n);
        }
        let sel = if x == 0 { &self.sel0 } else { &self.sel1 };
        let il = n / WORD_SIZE;
        let is = n % WORD_SIZE;
        match &sel[il] {
            Sparse(dir) => dir[is],
            Dense(range) => {
                let mut lo = range.start / WORD_SIZE;
                let mut hi = 1 + (range.end - 1) / WORD_SIZE;
                while hi - lo > 1 {
                    let mid = lo + (hi - lo) / 2;
                    let rank = self.rank_rough(mid, x);
                    *(if rank <= n { &mut lo } else { &mut hi }) = mid;
                }
                let rank_frac = n - self.rank_rough(lo, x);
                lo * WORD_SIZE
                    + Self::find_nth_small(self.buf[lo], x, rank_frac)
            }
        }
    }
    fn rank_rough(&self, n: usize, x: u64) -> usize {
        let rank1 = self.rank[n];
        let rank = if x == 0 { n * WORD_SIZE - rank1 } else { rank1 };
        rank
    }
    fn find_nth_small(word: u64, x: u64, n: usize) -> usize {
        let mut word = if x == 0 { !word } else { word };
        let mut n = n as u32;
        let mut res = 0;
        for &mid in &[32, 16, 8, 4, 2, 1] {
            let count = (word & !(!0 << mid)).count_ones();
            if count <= n {
                n -= count;
                word >>= mid;
                res += mid;
            }
        }
        res
    }
}

#[test]
fn select_internal() {
    assert_eq!(RsDict::find_nth_small(0x00000000_00000001_u64, 1, 0), 0);
    assert_eq!(RsDict::find_nth_small(0x00000000_00000003_u64, 1, 1), 1);
    assert_eq!(RsDict::find_nth_small(0x00000000_00000010_u64, 1, 0), 4);
    assert_eq!(RsDict::find_nth_small(0xffffffff_ffffffff_u64, 1, 63), 63);
}

#[test]
fn test_rs() {
    let n = 65536 + 4096;
    let buf: Vec<_> = (0..n).map(|i| i % 1024 != 0).collect();

    let rs: RsDict = buf.clone().into();
    let mut zero = 0;
    let mut one = 0;
    for i in 0..n {
        assert_eq!(rs.count(0..i, 0), zero);
        assert_eq!(rs.count(0..i, 1), one);
        if buf[i] {
            one += 1;
        } else {
            zero += 1;
        }
    }
    assert_eq!(rs.count(.., 0), zero);
    assert_eq!(rs.count(.., 1), one);

    let zeros: Vec<_> = (0..n).filter(|&i| !buf[i]).collect();
    let ones: Vec<_> = (0..n).filter(|&i| buf[i]).collect();

    for i in 0..zeros.len() {
        let s0 = rs.find_nth(.., 0, i);
        assert_eq!(s0, Some(zeros[i]));
        assert_eq!(rs.count(..=s0.unwrap(), 0), i + 1);
    }
    for i in 0..ones.len() {
        let s1 = rs.find_nth(.., 1, i);
        assert_eq!(s1, Some(ones[i]));
        assert_eq!(rs.count(..=s1.unwrap(), 1), i + 1);
    }
    assert_eq!(rs.find_nth(.., 0, zeros.len()), None);
    assert_eq!(rs.find_nth(.., 1, ones.len()), None);
}
