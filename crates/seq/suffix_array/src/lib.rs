//! 接尾辞配列。

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::ops::Index;

/// 接尾辞配列。
///
/// 文字列 $S$ の各接尾辞を辞書順でソートしたもの。
/// より正確には、$i$ ($0\\le i\\le |S|$) を $S[i\\dots]$ をキーとしてソートした配列 $A$ である。
///
/// 内部では高さ配列 (LCPA; Longest Common Prefix Array) $L$ も持っている。
/// $L[i]$ ($0\\le i < |S|$) は $S[A[i]\\dots]$ と $S[A[i+1]\\dots]$ の最長共通接頭辞の長さである。
///
/// # Idea
/// そのうち書く：
///
/// - SA-IS の概要
/// - SA/LCPA を用いた文字列検索の概要
///
/// ## See also
/// [CS166](http://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/04/Slides04.pdf)。
/// 差分スライドの関係でページ数がめちゃくちゃ多くて重いので注意。軽いのは
/// [こっち](http://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/04/Small04.pdf)。
///
/// # Complexity
/// 入力中の文字の種類を $\\sigma$、文字列長を $n$ とする。
/// SA-IS を用いて構築するため、前処理は $O(\\sigma\\log(\\sigma)+n)$ 時間。
///
/// 検索は、パターン長を $m$ として $O(m\\log(n))$ 時間。
///
/// # Notes
/// 工夫をすると、検索は $O(m+\\log(n))$ 時間にできるらしい？
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuffixArray<T: Ord> {
    buf: Vec<T>,
    sa: Vec<usize>,
    lcpa: Vec<usize>,
}

impl<T: Clone + Ord, S: AsRef<[T]>> From<S> for SuffixArray<T> {
    fn from(buf: S) -> Self {
        let buf: Vec<_> = buf.as_ref().to_vec();
        let buf_usize = compress(&buf);
        let sa = sa_is(&buf_usize);
        let lcpa = make_lcpa(&sa, &buf_usize[0..buf.len()]);
        Self { buf, sa, lcpa }
    }
}

fn compress<T: Clone + Ord>(buf: &[T]) -> Vec<usize> {
    let enc: BTreeSet<_> = buf.iter().cloned().collect();
    let enc: BTreeMap<_, _> =
        enc.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    buf.iter()
        .map(|x| enc[x] + 1)
        .chain(std::iter::once(0)) // for '$'
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LsType {
    LType,
    SType(bool), // true if leftmost S-type
}
use LsType::*;

fn count_freq(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().for_each(|&x| res[x] += 1);
    res
}

fn inv_perm(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().enumerate().for_each(|(i, &x)| res[x] = i);
    res
}

fn ls_classify(buf: &[usize]) -> Vec<LsType> {
    let mut res = vec![SType(false); buf.len()];
    for i in (0..buf.len() - 1).rev() {
        res[i] = match buf[i].cmp(&buf[i + 1]) {
            Less => SType(false),
            Equal if res[i + 1] == SType(false) => SType(false),
            _ => LType,
        };
    }
    for i in 1..buf.len() {
        if let (LType, SType(_)) = (res[i - 1], res[i]) {
            res[i] = SType(true);
        }
    }
    res
}

fn bucket_head(count: &[usize]) -> Vec<usize> {
    let mut head = [&[0_usize], &count[0..count.len() - 1]].concat().to_vec();
    for i in 1..head.len() {
        head[i] += head[i - 1];
    }
    head
}

fn bucket_tail(count: &[usize]) -> Vec<usize> {
    let mut tail = count.to_vec();
    for i in 1..count.len() {
        tail[i] += tail[i - 1];
    }
    tail
}

fn induce(
    buf: &[usize],
    sa: &mut [Option<usize>],
    count: &[usize],
    ls: &[LsType],
) {
    let mut head = bucket_head(count);
    for i in 0..sa.len() {
        match sa[i] {
            Some(j) if j > 0 && ls[j - 1] == LType => {
                sa[head[buf[j - 1]]] = Some(j - 1);
                head[buf[j - 1]] += 1;
            }
            _ => {}
        }
    }
    let mut tail = bucket_tail(count);
    for i in (1..count.len()).rev() {
        match sa[i] {
            Some(j) if j > 0 && ls[j - 1] != LType => {
                tail[buf[j - 1]] -= 1;
                sa[tail[buf[j - 1]]] = Some(j - 1);
            }
            _ => {}
        }
    }
}

fn reduce(buf: &[usize], lms: Vec<usize>, ls: &[LsType]) -> Vec<usize> {
    if lms.len() <= 1 {
        return vec![0; lms.len()];
    }

    let e = |(i0, i1)| {
        if (ls[i0], ls[i1]) == (SType(true), SType(true)) {
            Some(true)
        } else if ls[i0] != ls[i1] || buf[i0] != buf[i1] {
            Some(false)
        } else {
            None
        }
    };

    let mut map = vec![0; buf.len()]; // map[lms[0]] = 0
    map[lms[1]] = 1;
    let mut x = 1;
    for i in 2..lms.len() {
        let equiv = buf[lms[i]] == buf[lms[i - 1]]
            && (lms[i] + 1..).zip(lms[i - 1] + 1..).find_map(e).unwrap();
        if !equiv {
            x += 1;
        }
        map[lms[i]] = x;
    }

    (0..buf.len())
        .filter_map(|i| match ls[i] {
            SType(true) => Some(map[i]),
            _ => None,
        })
        .collect()
}

fn sa_is(buf: &[usize]) -> Vec<usize> {
    let len = buf.len();
    let count = count_freq(buf);
    if count.iter().all(|&x| x == 1) {
        return inv_perm(buf);
    }

    let ls = ls_classify(buf);
    let mut sa = vec![None; len];
    let mut tail = bucket_tail(&count);
    for i in (1..len).rev().filter(|&i| ls[i] == SType(true)) {
        tail[buf[i]] -= 1;
        sa[tail[buf[i]]] = Some(i);
    }
    induce(buf, &mut sa, &count, &ls);

    let lms: Vec<_> = sa
        .into_iter()
        .map(std::option::Option::unwrap)
        .filter(|&i| ls[i] == SType(true))
        .collect(); // in lexicographic order
    let rs_sa = sa_is(&reduce(buf, lms, &ls));

    // in appearing order
    let lms: Vec<_> = (0..len).filter(|&i| ls[i] == SType(true)).collect();

    let mut tail = bucket_tail(&count);
    let mut sa = vec![None; len];
    for i in rs_sa.into_iter().rev() {
        let j = lms[i];
        tail[buf[j]] -= 1;
        sa[tail[buf[j]]] = Some(j);
    }
    induce(buf, &mut sa, &count, &ls);

    sa.into_iter().map(std::option::Option::unwrap).collect()
}

fn make_lcpa(sa: &[usize], buf: &[usize]) -> Vec<usize> {
    let len = buf.len();
    let rank = inv_perm(&sa);
    let mut h = 0;
    let mut lcpa = vec![0_usize; len];
    for i in 0..len {
        let j = sa[rank[i] - 1];
        h = (h.max(1) - 1..)
            .find(|&h| match (buf.get(i + h), buf.get(j + h)) {
                (Some(x), Some(y)) => x != y,
                _ => true,
            })
            .unwrap();
        lcpa[rank[i] - 1] = h;
    }
    lcpa
}

impl<T: Ord> SuffixArray<T> {
    pub fn search<'a, S: AsRef<[T]>>(&'a self, pat: S) -> &'a [usize] {
        let pat = pat.as_ref();
        let lo = {
            let mut lt = 1_usize.wrapping_neg();
            let mut ge = self.sa.len();
            while ge.wrapping_sub(lt) > 1 {
                let mid = lt.wrapping_add(ge.wrapping_sub(lt) / 2);
                let pos = self.sa[mid];
                match self.buf[pos..].cmp(pat) {
                    Less => lt = mid,
                    _ => ge = mid,
                }
            }
            ge
        };
        if lo >= self.sa.len() {
            return &self.sa[lo..lo];
        }
        let hi = {
            let mut le = lo.wrapping_sub(1);
            let mut gt = self.sa.len();
            while gt.wrapping_sub(le) > 1 {
                let mid = le.wrapping_add(gt.wrapping_sub(le) / 2);
                let pos = self.sa[mid];
                let len = pat.len().min(self.buf[pos..].len());
                match self.buf[pos..pos + len].cmp(pat) {
                    Greater => gt = mid,
                    _ => le = mid,
                }
            }
            gt
        };
        &self.sa[lo..hi]
    }
}

impl<T: Ord> Index<usize> for SuffixArray<T> {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        &self.sa[i]
    }
}

impl<T: Ord> From<SuffixArray<T>> for Vec<usize> {
    fn from(sa: SuffixArray<T>) -> Self {
        sa.sa
    }
}
