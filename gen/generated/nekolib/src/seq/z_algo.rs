//! Z algorithm。

use std::fmt::Debug;
use std::ops::Range;

/// Z algorithm。
///
/// 文字列 $S$ について、$Z\[i\]$ ($0\\le i < |S|$) が
/// $S$ と $S\[i\\dots\]$ の最長共通接頭辞の長さであるような配列 $Z$ を構築する。
///
/// # Implementation notes
/// テキスト `T` 中のパターン `P` を探すとき、`T` と `P` に含まれない文字 `'$'`
/// を用いて作った文字列 `P + '$' + T` の Z value を計算することで求められる。
///
/// 一方、この方法においては `'$'` を適切に探す必要がある。
/// caller 側が探すのは面倒・バグの元であり、callee 側が探すのはコストがかかる。
/// あるいは、そういう元が存在しない場合もなくはない。
///
/// さて、`P + '$' + T` を読み込んだ場合の挙動は `'$'` を仮定しなくても模倣できる。
/// よって、多少の実装量には目をつぶりつつそういう実装にした。
/// テキストがパターン構築時には与えられないような場合や、
/// 複数の処理がオンラインに与えられる場合にも対応しやすいと思う。
///
/// # Suggestions
/// [`std::str::matches`](https://doc.rust-lang.org/std/primitive.str.html#method.matches)
/// の実装を参考にするなどして、`fn occurrences(&self)` を持つような trait
/// を作るとよさそう。テストを KMP に対して再利用できるので。
///
/// # Complexity
/// 構築は $O(|S|)$ 時間。検索は、テキスト $T$ に対して $O(|T|)$ 時間。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZSearcher<T: Eq> {
    pat: Vec<T>,
    z: Vec<usize>,
}

impl<T: Clone + Eq> From<Vec<T>> for ZSearcher<T> {
    fn from(pat: Vec<T>) -> Self {
        let len = pat.len();
        let mut z = vec![len; len];
        let mut i = 1;
        let mut j = 0;
        while i < len {
            while i + j < len && pat[j] == pat[i + j] {
                j += 1;
            }
            z[i] = j;
            if j == 0 {
                i += 1;
                continue;
            }
            let mut k = 1;
            while i + k < len && k + z[k] < j {
                z[i + k] = z[k];
                k += 1;
            }
            i += k;
            j -= k;
        }
        Self { pat, z }
    }
}

impl<T: Eq> ZSearcher<T> {
    pub fn occurrences<'a>(&'a self, s: &'a [T]) -> Occurrences<'a, T> {
        Occurrences { text_index: 0, match_len: 0, z: &self, text: s }
    }

    pub fn z(&self, i: usize) -> usize { self.z[i] }
}

pub struct Occurrences<'a, T: Eq> {
    text_index: usize,
    match_len: usize,
    z: &'a ZSearcher<T>,
    text: &'a [T],
}

impl<T: Eq> Iterator for Occurrences<'_, T> {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let text = self.text;
        let pat = &self.z.pat;
        let z = &self.z.z;

        if pat.is_empty() {
            return if self.text_index < text.len() {
                let i = self.text_index;
                self.text_index += 1;
                Some(i..i)
            } else {
                None
            };
        }

        let mut i = self.text_index;
        let mut j = self.match_len;
        while i < text.len() {
            while i < text.len() && j < pat.len() && pat[j] == text[i] {
                i += 1;
                j += 1;
            }
            if j == 0 {
                i += 1;
                continue;
            }
            let mut k = 1;
            while k < pat.len() && k + z[k] < j {
                k += 1;
            }
            if j == pat.len() {
                self.text_index = i;
                self.match_len = j - k;
                return Some(i - j..i);
            }
            j -= k;
        }
        self.text_index = text.len();
        None
    }
}
