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
#[derive(Debug)]
pub struct RsDict {
    len: usize,
    buf: Vec<u64>,
    rank: Vec<usize>,
    sel0: Vec<usize>,
    sel1: Vec<usize>,
    ssel0: Vec<Vec<usize>>,
    ssel1: Vec<Vec<usize>>,
}

impl From<Vec<bool>> for RsDict {
    fn from(buf: Vec<bool>) -> Self {
        let len = buf.len();
        let buf = Self::compress_vec_bool(buf);
        let rank = Self::preprocess_rank(&buf);
        let (sel0, ssel0) = Self::preprocess_select(&buf, len, 0);
        let (sel1, ssel1) = Self::preprocess_select(&buf, len, 1);
        Self { len, buf, rank, sel0, sel1, ssel0, ssel1 }
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
    ) -> (Vec<usize>, Vec<Vec<usize>>) {
        let mut sel = vec![0];
        let mut ssel = vec![];
        let mut tmp = vec![];
        for i in 0..n {
            let c = buf[i / WORD_SIZE] >> (i % WORD_SIZE) & 1;
            if c != x {
                continue;
            }
            tmp.push(i + 1);
            if tmp.len() < WORD_SIZE {
                continue;
            }
            let len = i + 1 - *sel.last().unwrap();
            sel.push(i + 1);
            ssel.push(if len < WORD_SIZE_2 { vec![] } else { tmp });
            tmp = vec![];
        }
        if let Some(&last) = tmp.last() {
            sel.push(last);
            ssel.push(tmp);
        }
        (sel, ssel)
    }
    pub fn rank(&self, end: usize, x: u64) -> usize {
        let il = end / WORD_SIZE;
        let is = end % WORD_SIZE;
        let rank1 = self.rank[il]
            + (self.buf[il] & !(!0_u64 << is)).count_ones() as usize;
        let rank = if x == 0 { end - rank1 } else { rank1 };
        rank
    }
    pub fn select(&self, x: u64, k: usize) -> usize {
        if self.rank(self.len, x) < k {
            panic!("the number of {}s is less than {}", x, k);
        }
        if k == 0 {
            return 0;
        }
        let k = k - 1;
        let sel = if x == 0 { &self.sel0 } else { &self.sel1 };
        let ssel = if x == 0 { &self.ssel0 } else { &self.ssel1 };
        let il = k / WORD_SIZE;
        let is = k % WORD_SIZE;
        if !ssel[il].is_empty() {
            return ssel[il][is];
        }
        let mut lo = sel[il];
        let mut hi = *sel.get(il + 1).unwrap_or(&self.len);
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let rank = self.rank(mid, x);
            if rank <= k {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        hi
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
            Some(self.select(x, offset + n + 1) - 1)
        }
    }
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

    eprintln!("{:?}", zeros);

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
