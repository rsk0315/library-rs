//! KMP 法。

use std::fmt::Debug;
use std::ops::Range;

use push_pop::{PopBack, PushBack};

/// KMP 法 (Knuth–Morris–Pratt algorithm)。
///
/// 文字列 $S$ を入力とする。
/// 各 $S[\\dots i]$ ($0\\le i\\le |S|$) の最長 border と最長 tagged border
/// の長さを求める。該当する border が存在しない要素は $-1$ とする。
///
/// 文字列 $S[\\dots i]$ の _border_ とは、$S[\\dots i]$ の真部分文字列であり、$S[\\dots i]$
/// の接頭辞かつ接尾辞であるような文字列である。
///
/// 文字列 $S[\\dots i]$ の border $S[\\dots j]$ ($0\\le j < i$) が $S[j] \\neq S[i]$
/// を満たすとき、$S[\\dots j]$ は $S[\\dots i]$ の _tagged border_ (_strict border_,
/// _strong border_) であると言う。ただし、$S[|S|]$ は $S$ 中に含まれない文字として定義する。
///
/// ```text
///     0 1 2 3 4 5 6 7 8   9
///   +-------------------+-------+
/// S | a a b a c a a b a | c ... |
///   +-------------------+-------+
/// ```
///
/// この例において、$S[\\dots 4] = \\mathtt{aaba}$ は $S[\\dots 9] = \\mathtt{aabacaaba}$
/// の border だが tagged border ではない ($S[4] = S[9] = \\mathtt{c}$)。一方、
/// $S[\\dots 2] = \\mathtt{aa}$ は tagged border である
/// ($S[2] = \\mathtt{b} \\neq S[9] = \\mathtt{c}$)。
///
/// この tagged border を用いることで、パターン検索を高速に行う。
///
/// # Implementation notes
/// パターンが静的であれば最長 border を求める必要はない。
/// パターンの末尾に対する push/pop を行える実装になっており、その更新の際に最長
/// border の長さが必要になる（はず）。
///
/// # See also
/// - <https://snuke.hatenablog.com/entry/2014/12/01/235807>
/// - <https://snuke.hatenablog.com/entry/2017/07/18/101026>
/// - <https://potetisensei.hatenablog.com/entry/2017/07/10/174908>
/// - <http://www-igm.univ-mlv.fr/~lecroq/string/node8.html>
///
/// # Complexity
/// 構築は $O(|S|) 時間。パターン末尾における更新は $O(\\log(|S|))$ 時間。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KmpSearcher<T: Eq> {
    pat: Vec<T>,
    fail: Vec<usize>,
    fail1: Vec<usize>,
}

impl<T: Clone + Eq> From<Vec<T>> for KmpSearcher<T> {
    fn from(pat: Vec<T>) -> Self {
        let len = pat.len();
        let fail1 = vec![1_usize.wrapping_neg(); len + 1];
        let fail = fail1.clone();
        let mut self_ = Self { pat, fail1, fail };
        for i in 0..self_.pat.len() {
            let (f, f1) = self_.calc_fail(i);
            self_.fail1[i + 1] = f1;
            self_.fail[i + 1] = f;
        }
        self_
    }
}

impl<T: Eq> KmpSearcher<T> {
    fn calc_fail(&self, i: usize) -> (usize, usize) {
        let pat_i = &self.pat[i];
        let mut j = self.fail1[i];
        while j < self.pat.len() && pat_i != &self.pat[j] {
            j = self.fail[j];
        }
        j = j.wrapping_add(1);
        match self.pat.get(i + 1) {
            Some(pat_ni) if pat_ni == &self.pat[j] => (self.fail[j], j),
            _ => (j, j),
        }
    }

    pub fn occurrences<'a>(&'a self, s: &'a [T]) -> Occurrences<'a, T> {
        Occurrences {
            text_index: 0,
            pat_index: 0,
            kmp: &self,
            text: s,
        }
    }
}

pub struct Occurrences<'a, T: Eq> {
    text_index: usize,
    pat_index: usize,
    kmp: &'a KmpSearcher<T>,
    text: &'a [T],
}

impl<T: Eq> Iterator for Occurrences<'_, T> {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let text = self.text;
        let pat = &self.kmp.pat;

        if pat.is_empty() {
            return if self.text_index < text.len() {
                let i = self.text_index;
                self.text_index += 1;
                Some(i..i)
            } else {
                None
            };
        }

        let mut j = self.pat_index;
        for (i, c) in text[self.text_index..].iter().enumerate() {
            let i = i + self.text_index;
            while j < pat.len() && &pat[j] != c {
                j = self.kmp.fail[j];
            }
            j = j.wrapping_add(1);
            if j == pat.len() {
                let e = i + 1;
                let res = e - pat.len()..e;
                self.text_index = e;
                self.pat_index = self.kmp.fail[j];
                return Some(res);
            }
        }
        self.text_index = text.len();
        None
    }
}

impl<T: Eq> PushBack for KmpSearcher<T> {
    type Input = T;
    fn push_back(&mut self, x: T) {
        let len = self.pat.len();
        self.pat.push(x);
        if len > 0 {
            *self.fail.last_mut().unwrap() = self.calc_fail(len - 1).0;
        }
        let (f, f1) = self.calc_fail(len);
        self.fail1.push(f1);
        self.fail.push(f);
    }
}

impl<T: Eq> PopBack for KmpSearcher<T> {
    type Output = usize;
    fn pop_back(&mut self) -> Option<usize> {
        if self.pat.is_empty() {
            None
        } else {
            self.pat.pop();
            self.fail1.pop();
            let res = self.fail.pop();
            *self.fail.last_mut().unwrap() = *self.fail1.last().unwrap();
            res
        }
    }
}
