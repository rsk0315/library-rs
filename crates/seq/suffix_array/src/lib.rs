//! 接尾辞配列。

use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::ops::Index;

/// 接尾辞配列。
///
/// 文字列 $S$ の各接尾辞を辞書順でソートしたもの。
/// より正確には、$i$ ($0\\le i\\le |S|$) を $S\[i\\dots\]$ をキーとしてソートした配列 $A$ である。
///
/// 内部では高さ配列 (LCPA; Longest Common Prefix Array) $L$ も持っている。
/// $L\[i\]$ ($0\\le i < |S|$) は $S\[A\[i\]\\dots\]$ と $S\[A\[i+1\]\\dots\]$ の最長共通接頭辞の長さである。
///
/// # Idea
/// ## 用語の定義など
///
/// 文字列 $S$ について、 $S\[|S|\]$ には辞書順最小の文字が入っていると見なす。
/// また、$S\[|S|+1\]$ には辞書順最大の文字が入っていると見なしている気がする。
///
/// 以下、$S\[i\\dots\]$ を接尾辞 $i$ と呼ぶ。
/// また、例として文字列 `GTCCCGATGTCATGTCAGGA$` を考える。
///
/// ```text
///  G  T  C  C  C  G  A  T  G  T  C  A  T  G  T  C  A  G  G  A  $
///  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
/// ```
///
/// この文字列の接尾辞配列は次のようになる。
///
/// ```text
///  20 19 16 11  6 15 10  2  3  4 18  5 17 13  8  0 14  9  1 12  7
/// ```
///
/// ### S-type と L-type
///
/// まず、接尾辞に対して _S-type_ と _L-type_ を定義する。
/// 接尾辞 $i$ が接尾辞 $i+1$ より辞書順で小さい (smaller)
/// とき、接尾辞 $i$ を S-type であると言う。そうでない (larger) とき、L-type
/// であると言う。長さが異なるので、等しくなることはないことに注意せよ。
/// なお、接尾辞 $|S|$ は S-type とする。
///
/// $i$ ($0\\le i \\lt |S|$) について次が成り立つので、接尾辞 $|S|$ が S-type
/// であることと合わせて、末尾から順に線形時間で求められる。
///
/// - $S\[i\] \\lt S\[i+1\]$ なら、接尾辞 $i$ は S-type。
/// - $S\[i\] \\gt S\[i+1\]$ なら、接尾辞 $i$ は L-type。
/// - $S\[i\] = S\[i+1\]$ なら、接尾辞 $i$ と接尾辞 $i+1$ は同じ type。
///
/// ### LMS suffix と LMS block
///
/// 接尾辞 $i-1$ が L-type であるような S-type の接尾辞 $i$ を、特に
/// LMS (leftmost S-type) suffix と言う。接尾辞 $0$ は LMS ではないことに注意せよ。
///
/// また、LMS suffix を、次の LMS suffix（存在すれば）の開始位置で区切ったものを
/// LMS block と呼ぶ。LMS block は次のようになる。
///
/// ```text
///  G  T  C  C  C  G  A  T  G  T  C  A  T  G  T  C  A  G  G  A  $
///  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
///  S  L *S  S  S  L *S  L *S  L  L *S  L *S  L  L *S  L  L  L *S
///       [------------]
///                   [------]
///                         [---------]
///                                  [------]
///                                        [---------]
///                                                 [------------]
///                                                             []
/// ```
///
/// ### バケット
///
/// 接尾辞配列を再掲する。ただし、接尾辞 $i$ に対して、その type と $S\[i\]$ を併記する。
///
/// ```text
///  20 19 16 11  6 15 10  2  3  4 18  5 17 13  8  0 14  9  1 12  7
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  L *S *S *S  L  L *S  S  S  L  L  L *S *S  S  L  L  L  L  L
/// ```
///
/// 同じ文字で始まる接尾辞が連続していることに気づく。この一連の部分をバケットと呼ぶ。
/// たとえば、`19`, `16`, `11`, `6` の部分を指して `A` のバケットと呼ぶことにする。
///
/// さらに、各文字のバケットにおいて、L-type が前半に、S-type が後半に来ていることにも気づく。
/// これは、$S\[i\] = S\[j\] = c$ であるような接尾辞 $i$ (L-type) と $j$ (S-type)
/// を考えたとき、接尾辞 $i$ は $c$ が 1 つ以上続いた後に $c$ より小さい文字が現れ、
/// 接尾辞 $j$ は $c$ が 1 つ以上続いた後に $c$ より大きい文字が現れることから従う。
///
/// ### Induced sorting
///
/// LMS suffix が適当な順で得られているとき、残りの接尾辞のソート順を求める。
/// 各 LMS suffix は対応するバケットの末尾に入っているとする。
///
/// まず、バケットを前から走査する。接尾辞 $i$ を見つけたとき、
/// 接尾辞 $i-1$ が L-type であれば、対応するバケットに入れる。
/// このとき、バケットを末尾から見て最初の空きに入れる。
///
/// 次に、バケットから S-type の接尾辞を取り除き、後ろから走査する。
/// 接尾辞 $i$ を見つけたとき、接尾辞 $i-1$ が S-type であれば、
/// 対応するバケットに入れる。このとき、バケットを先頭から見て最初の空きに入れる。
///
/// LMS suffix を正しい順で与えれば、全体として正しいソート順が得られる。
/// 入力として与えた LMS suffix の順が保たれるとは限らなさそう。
/// わからないことが多い。
///
/// suffix のソートではなく、次の LMS block の開始位置までの substring
/// のソートと見る方が妥当そうな気がしてきたので、見直します。`todo!()`
///
/// ## アルゴリズムの概要
///
/// まず、$S$ を末尾から走査し、type を求める。
///
/// ```text
///  G  T  C  C  C  G  A  T  G  T  C  A  T  G  T  C  A  G  G  A  $
///  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
///  S  L *S  S  S  L *S  L *S  L  L *S  L *S  L  L *S  L  L  L *S
/// ```
///
/// ただし、`*S` は LMS を表す。
///
/// 次に、LMS を出現順にバケットの末尾に置いて induced sorting を行う。
///
/// ```text
///  20  -  6 11 16  -  -  -  -  2  -  -  -  -  8 13  -  -  -  -  -
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  - *S *S *S  -  -  -  - *S  -  -  -  - *S *S  -  -  -  -  -
/// ```
///
/// ```text
///  20 19  6 11 16 10 15  -  -  2 18  5 17  -  8 13  9 14  1  7 12
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  L *S *S *S  L  L  -  - *S  L  L  L  - *S *S  L  L  L  L  L
/// ```
///
/// ```text
///  20 19 16  6 11 10 15  2  3  4 18  5 17  8 13  0  9 14  1  7 12
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  L *S *S *S  L  L *S  S  S  L  L  L *S *S  S  L  L  L  L  L
/// ```
///
/// この時点では各接尾辞の正しいソート順は得られていないが、次の LMS block
/// 先頭以降の文字を無視すれば、ソート順に並んでいる。
///
/// ```text
///  : ...
/// 16 AGGA$
///  6 ATG   // TCATG ...
/// 11 ATG   // TCAGG ...
/// 10 CA    // TGTCA ...
/// 15 CA    // GGA$
///  2 CCCGA // TGTCA ...
///  3 CCGA  // TGTCA ...
///  4 CGA   // TGTCA ...
/// 18 GA$
///  : ...
/// ```
///
/// 特に、LMS block の正しいソート順は得られている。
/// `*S` の添字を得られた順に並べると `20 16 6 11 2 8 13` となり、対応する
/// LMS block を順に並べると次のようになる。
///
/// ```text
/// $ AGGA$ ATG ATG CCCGA GTCA GTCA
/// ```
///
/// これはソート順で得られているので、隣同士の要素を比較するだけで、各要素の順序づけがわかる。
/// 等しい要素があることに注意せよ。
///
/// ```text
///  G  T  C  C  C  G  A  T  G  T  C  A  T  G  T  C  A  G  G  A  $
///  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
///  S  L *S  S  S  L *S  L *S  L  L *S  L *S  L  L *S  L  L  L *S
///       [----------][----][-------][----][-------][----------][]
///                  3     2        4     2        4           1 0
/// ```
///
/// この順序づけによってできる列 (_reduced string_) `3 2 4 2 4 1 0` の接尾辞配列は
/// 再帰的に SA-IS によって求めることができ、`6 5 3 1 0 4 2` となる。
/// これを元の文字列に対応させることで、LMS suffix のソート順が `20 16 11 6 2 13 8`
/// であるとわかり、これを用いて再度 induced sorting を行う。
///
/// ```text
///  20  - 16 11  6  -  -  -  -  2  -  -  -  - 13  8  -  -  -  -  -
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  - *S *S *S  -  -  -  - *S  -  -  -  - *S *S  -  -  -  -  -
/// ```
///
/// ```text
///  20 19 16 11  6 15 10  -  -  2 18  5 17  - 13  8 14  9  1 12  7
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  L *S *S *S  L  L  -  - *S  L  L  L  - *S *S  L  L  L  L  L
/// ```
///
/// ```text
///  20 19 16 11  6 15 10  2  3  4 18  5 17 13  8  0 14  9  1 12  7
///   $  A  A  A  A  C  C  C  C  C  G  G  G  G  G  G  T  T  T  T  T
///  *S  L *S *S *S  L  L *S  S  S  L  L  L *S *S  S  L  L  L  L  L
/// ```
///
/// これにより、所望の接尾辞配列が得られる。
///
/// なお、reduced string の接尾辞配列は SA-IS を再帰的に呼ぶことで得られる。
/// 再帰の base case は、列の各要素が distinct な場合で、逆順列が接尾辞配列となる。
///
/// 連続して LMS suffix が現れないことと、先頭位置は LMS にならないことから、
/// reduced string は元の文字列の半分以下の長さになることがわかる。
/// よって、長さ $n$ での計算量を $T(n)$ とおくと、次のようになる。
/// $$ T(n) \\le T(n/2) + T(n/4) + \\dots + T(1) + O(n) \\in O(n). $$
///
/// ## SA/LCPA を用いた文字列検索
///
/// `todo!()`
///
/// # Complexity
/// 入力中の文字の種類を $\\sigma$、文字列長を $n$ とする。
/// SA-IS を用いて構築するため、前処理は $O(\\sigma\\log(\\sigma)+n)$ 時間。
///
/// 検索は、パターン長を $m$ として $O(m\\log(n))$ 時間。
///
/// # Notes
/// 工夫をすると、検索は $O(m+\\log(n))$ 時間にできるらしい？
///
/// # References
/// - Nong, Ge, Sen Zhang, and Wai Hong Chan. "Two efficient algorithms for linear time suffix array construction." _IEEE Transactions on Computers_ 60, no. 10 (2010): 1471-1484.
/// - Ko, Pang, and Srinivas Aluru. "Space efficient linear time construction of suffix arrays." In _Annual Symposium on Combinatorial Pattern Matching_, pp. 200-210. Springer, Berlin, Heidelberg, 2003.
///
/// ## See also
/// [CS166](http://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/04/Slides04.pdf)。
/// 差分スライドの関係でページ数がめちゃくちゃ多くて重いので注意。軽いのは
/// [こっち](http://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/04/Small04.pdf)。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuffixArray<T: Ord> {
    buf: Vec<T>,
    sa: Vec<usize>,
    lcpa: Vec<usize>,
}

impl<T: Ord> From<Vec<T>> for SuffixArray<T> {
    fn from(buf: Vec<T>) -> Self {
        let buf_usize = hash(&buf);
        let sa = sa_is(&buf_usize);
        let lcpa = make_lcpa(&sa, &buf_usize[..buf.len()]);
        Self { buf, sa, lcpa }
    }
}

impl From<&str> for SuffixArray<char> {
    fn from(buf: &str) -> Self {
        Self::from(buf.chars().map(|c| c).collect::<Vec<_>>())
    }
}

/// 座標圧縮をする。
///
/// `buf` の末尾に辞書順最小の文字 `$` を付加した列を、座標圧縮して返す。
fn hash<T: Ord>(buf: &[T]) -> Vec<usize> {
    let enc: BTreeSet<_> = buf.iter().collect();
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
    SType(bool), // true iff leftmost S-type
}
use LsType::{LType, SType};

/// 出現回数を求める。
///
/// `res[x]` が `buf` における `x` の出現回数となる配列 `res` を返す。
///
/// # Requirements
/// `buf` の要素は `0..buf.len()` に含まれる。
fn count_freq(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().for_each(|&x| res[x] += 1);
    res
}

/// 逆順列を返す。
///
/// `res[x]` が `buf` における `x` の出現位置となる配列 `res` を返す。
///
/// # Requirements
/// `buf` の要素は `0..buf.len()` に含まれ、かつ distinct である。
fn inv_perm(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().enumerate().for_each(|(i, &x)| res[x] = i);
    res
}

/// LS type を求める。
///
/// `res[i]` が `buf[i]` の LS type である配列 `res` を返す。
fn ls_classify(buf: &[usize]) -> Vec<LsType> {
    let mut res = vec![SType(false); buf.len()];
    for i in (0..buf.len() - 1).rev() {
        res[i] = match buf[i].cmp(&buf[i + 1]) {
            Less => SType(false),
            Equal => res[i + 1],
            Greater => LType,
        };
    }
    for i in 1..buf.len() {
        if let (LType, SType(_)) = (res[i - 1], res[i]) {
            res[i] = SType(true);
        }
    }
    res
}

/// 各バケットの開始位置を求める。
///
/// # Input
/// `count[i]` が `i` 番目のバケットのサイズである配列 `count`。
fn bucket_head(count: &[usize]) -> Vec<usize> {
    let n = count.len();
    let mut head: Vec<_> = std::iter::once(&0)
        .chain(&count[..n - 1])
        .cloned()
        .collect();
    for i in 1..n {
        head[i] += head[i - 1];
    }
    head
}

/// 各バケットの終端位置を求める。
///
/// # Input
/// `count[i]` が `i` 番目のバケットのサイズである配列 `count`。
fn bucket_tail(count: &[usize]) -> Vec<usize> {
    let mut tail = count.to_vec();
    for i in 1..count.len() {
        tail[i] += tail[i - 1];
    }
    tail
}

/// すでに判明している SA と LS type から、SA の残りの要素を求める。
fn induce(
    buf: &[usize],
    sa: &mut [Option<usize>],
    count: &[usize],
    ls: &[LsType],
) {
    let mut head = bucket_head(count);
    for i in 0..sa.len() {
        if let Some(j) = sa[i] {
            if j > 0 && ls[j - 1] == LType {
                sa[head[buf[j - 1]]] = Some(j - 1);
                head[buf[j - 1]] += 1;
            }
        }
    }
    let mut tail = bucket_tail(count);
    for i in (1..count.len()).rev() {
        if let Some(j) = sa[i] {
            if j > 0 && ls[j - 1] != LType {
                tail[buf[j - 1]] -= 1;
                sa[tail[buf[j - 1]]] = Some(j - 1);
            }
        }
    }
}

/// 各 LMS block を文字とする文字列を作る。
///
/// 新たな文字も辞書順に番号づけられる。
///
/// # Examples
/// `[CCCG][AT][GTC][AT][GTC][AGGA][$]` → `3242410`
fn reduce(buf: &[usize], lms: &[usize], ls: &[LsType]) -> Vec<usize> {
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

/// SA-IS により接尾辞配列を求める。
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
    let rs_sa = sa_is(&reduce(buf, &lms, &ls));

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
    let mut h = 0_usize;
    let mut lcpa = vec![0_usize; len];
    for i in 0..len {
        let j = sa[rank[i] - 1];
        h = (h.saturating_sub(1)..)
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
    pub fn search(&self, pat: &[T]) -> impl Iterator<Item = usize> + '_ {
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
            return self.sa[lo..lo].iter().cloned();
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
        self.sa[lo..hi].iter().cloned()
    }
}

impl<T: Ord> From<SuffixArray<T>> for Vec<usize> {
    fn from(sa: SuffixArray<T>) -> Self {
        sa.sa
    }
}

#[cfg(test)]
mod tests {
    use crate::SuffixArray;
    #[test]
    fn test_simple() {
        let buf: Vec<_> = "abracadabra".chars().collect();
        let sa: SuffixArray<_> = buf.into();
        let sa: Vec<_> = sa.into();
        assert_eq!(sa, [11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2])
    }

    #[test]
    fn test_lms() {
        let buf: Vec<_> = "GTCCCGATGTCATGTCAGGA".chars().collect();
        let sa: SuffixArray<_> = buf.into();
        let sa: Vec<_> = sa.into();
        assert_eq!(
            sa,
            [
                20, 19, 16, 11, 6, 15, 10, 2, 3, 4, 18, 5, 17, 13, 8, 0, 14, 9,
                1, 12, 7
            ]
        );
    }
}
