//! 配列上の三分探索。

/// 配列上の三分探索。
///
/// 配列 $(a\_0, a_1, \\dots, a\_{n-1})$ において、以下を満たす $i$ および $f(a\_i)$ を返す。
/// $$ f(a\_{i-1}) < f(a\_i) > f(a\_{i+1}). $$
///
/// # Requirements
/// 凸である。すなわち、ある $i$ が存在して以下の二つが成り立つ。
///
/// - ${}^\\forall j \\in [0, i)$ に対して $f(a\_j) < f(a\_{j+1})$。
/// - ${}^\\forall j \\in [i, n)$ に対して $f(a\_j) > f(a\_{j+1})$。
///
/// ただし、$f(a\_{-1}) = f(a\_n) = -\\infty$ と見なす。
///
/// # Complexity
/// $\\log_{\\varphi}(n)+O(1)$ 回の $f$ の呼び出しと比較を行う。
/// ただし $\\varphi$ は黄金比 $(1+\\sqrt{5})/2 = 1.618\\dots$ である。
///
/// 連続値の場合における黄金比分割のように、Fibonacci 数列に基づいて
/// 配列を分割していくため、関数値を使い回すことができる。
/// 三等分する素直な実装での呼び出し回数は
/// $2\\cdot\\log_{3/2}(n)+O(1)$ 回となる（係数の $2$ に注意）。
/// $$ \\sqrt{3/2} < 1.225 < 1.618 < \\varphi $$
/// であり、$\\log$ の底は大きい方がうれしいので、こちらの実装の方が定数倍が軽い。
///
/// # Suggestions
/// 引数に `&[T]` を渡すのではなく、`Range<usize>` を渡すことにして、
/// `F: Fn(usize) -> U, U: Ord` としておく方がよさそうに見える。
/// スライス上で行いたい場合は `|i| f(&buf[i])` を渡すことにすればよい。
///
/// また Requirements にも、定義されていない $a\_{-1}$ や $a\_n$ を用いて
/// $f(a\_{-1})$ や $f(a\_n)$ などと書く必要がなくなり、
/// $f(l-1) = f(r) = -\\infty$ と見なすと書けば十分になる。
///
/// # Examples
/// ```
/// use nekolib::algo::extremum;
///
/// let square = |&x: &i32| x * x;
/// assert_eq!(extremum(&[1, 3, 4, 6, 5, 2], square), (3_usize, 36));
/// ```
pub fn extremum<T, U, F>(buf: &[T], f: F) -> (usize, U)
where
    F: Fn(&T) -> U,
    U: Ord,
{
    let mut i0 = 0usize;
    let mut i1 = 1usize;
    let n = buf.len();
    while i0 + i1 < n {
        let tmp = i0 + i1;
        i0 = i1;
        i1 = tmp;
    }
    let g = |i| if i < n { Some(f(&buf[i])) } else { None };
    let mut d = i0;
    let mut g0 = g(i0);
    let mut g1 = g(i1);
    while d >= 1 {
        match (g0, g1) {
            (Some(f0), Some(f1)) if f0 < f1 => {
                let tmp = i0 + d;
                i0 = i1;
                i1 = tmp;
                g0 = Some(f1);
                g1 = g(i1);
            }
            (f0, _) => {
                let tmp = i1 - d;
                i1 = i0;
                i0 = tmp;
                g1 = f0;
                g0 = g(i0);
            }
        }
        d = d + i0 - i1;
    }
    match (g0, g1) {
        (Some(f0), Some(f1)) if f0 < f1 => (i1, f1),
        (Some(f0), _) => (i0, f0),
        (None, _) => unreachable!(),
    }
}
