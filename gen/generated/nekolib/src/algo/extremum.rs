//! 三分探索。

use std::ops::Range;

/// 三分探索で極値を探す。
///
/// # Notes
/// [`extremum`] を参照されたい。
///
/// # Examples
/// ```
/// use nekolib::algo::extremum_slice;
///
/// let buf = [1, 3, 4, 6, 5, 2, 0, 1, 3];
/// //         <------ f ------>
/// //                  <------ g ------>
///
/// let f = |&x: &i32| x * x;
/// assert_eq!(extremum_slice(&buf[..6], f), (3_usize, 36));
/// let g = |&x: &i32| -x;
/// assert_eq!(extremum_slice(&buf[3..], g), (3_usize, 0));
/// ```
pub fn extremum_slice<T: Ord>(
    buf: &[T],
    mut f: impl FnMut(&T) -> T,
) -> (usize, T) {
    extremum(0..buf.len(), |i| f(&buf[i]))
}

/// 三分探索で極値を探す。
///
/// 離散値の区間 $[l, r)$ において、以下を満たす $i$ および $f(i)$ を返す。
/// $$ f(i-1) < f(i)\\text{ and } f(i) > f(i+1). $$
///
/// # Requirements
/// 凸である。すなわち、ある $i$ が存在して以下の二つが成り立つ。
/// - ${}^\\forall j \\in [l, i)$ に対して $f(j) < f(j+1)$。
/// - ${}^\\forall j \\in [i, r-1)$ に対して $f(j) > f(j+1)$。
///
/// # Implementation notes
/// 連続値の場合における黄金比分割のように、Fibonacci
/// 数列に基づいて区間を分割していくため、関数値を使い回すことができる。
///
/// 関数 $f$ の呼び出し回数を、区間を三等分する素直な実装と比較する。
/// 三等分の実装では
/// $2\\cdot\\log_{3/2}(r-l)+O(1)$ 回（係数の $2$ に注意）であり、こちらは
/// $\\log_{\\varphi}(r-l)+O(1)$ 回である。
/// ただし $\\varphi$ は黄金比 $(1+\\sqrt{5})/2 = 1.618\\dots$ である。
/// $$ \\sqrt{3/2} < 1.225 < 1.618 < \\varphi $$
/// であり、$\\log$ の底は大きい方がうれしいので、こちらの実装の方が定数倍が軽い。
///
/// コード長はやや長くなるものの、単純な例での実測では三等分のものよりわずかに高速であった。
///
/// # Complexity
/// $F\_0 = 1$, $F\_1 = 2$, $F\_i = F\_{i-1} + F\_{i-2}$ ($i \\ge 2$) で定義される数列 $\\{F\_k\\}$ を考える。
/// 区間幅 $n$ がある $k$ に対して $n \\le F\_k$ と抑えられるとき、$f$ の呼び出しを高々
/// $k$ 回、関数値同士の比較を高々 $k-1$ 回行う。
/// なお、この $k$ は $\\lceil\\log\_{\\varphi}(n)\\rceil$ で抑えられる。
///
/// # Suggestions
/// 引数は `Range<usize>` を渡すことにしているものの、実際には
/// `RangeBounds<I: {integer}>` を渡せるようにする方がよさそう？
/// ただし、両端とも `Unbounded` であっては困りそう（特に多倍長を視野に入れる場合？）。
/// 多倍長だと `Copy` がないから、計算結果自体を使い回せても `.clone()` でつらい？
///
/// # Examples
/// ```
/// use nekolib::algo::extremum;
///
/// let buf = [1, 3, 4, 6, 5, 2, 0, 1, 3];
/// //         <------ f ------>
/// //                  <------ g ------>
///
/// let f = |i: usize| buf[i] * buf[i];
/// assert_eq!(extremum(0..6, f), (3_usize, 36));
/// let g = |i: usize| -buf[i];
/// assert_eq!(extremum(3..8, g), (6_usize, 0));
/// ```
///
/// ```
/// use nekolib::algo::extremum;
///
/// let n = 1500;
/// for k in 0..n {
///     let mut count = 0;
///     let f = |i| { count += 1; -(i as i32 - k as i32).abs() };
///     assert_eq!(extremum(0..n, f), (k, 0));
///     assert!(count <= 15);
/// }
/// ```
pub fn extremum<T: Ord>(
    Range { start, end }: Range<usize>,
    mut f: impl FnMut(usize) -> T,
) -> (usize, T) {
    let n = end - start;
    if n == 0 {
        panic!("range must be non-empty");
    }
    if n == 1 {
        return (start, f(start));
    }

    let mut i0 = 0;
    let mut i1 = 1;
    while i0 + i1 <= n {
        let tmp = i0 + i1;
        i0 = std::mem::replace(&mut i1, tmp);
    }
    let mut g = |i| if i <= n { Some(f(start + i - 1)) } else { None }; // None means -inf
    let mut d = i0;
    let mut g0 = g(i0);
    let mut g1 = g(i1);
    while d > 1 {
        match (g0, g1) {
            (Some(f0), Some(f1)) if f0 < f1 => {
                // |lo  i0 < i1     hi|
                //     |lo   i0 i1  hi|
                let tmp = i0 + d;
                i0 = std::mem::replace(&mut i1, tmp);
                g0 = Some(f1);
                g1 = g(i1);
            }
            (Some(f0), _) => {
                // |lo     i0 > i1  hi|
                // |lo  i0 i1   lo|
                let tmp = i1 - d;
                i1 = std::mem::replace(&mut i0, tmp);
                g1 = Some(f0);
                g0 = g(i0);
            }
            (None, _) => unreachable!(),
        }
        d -= i1 - i0;
    }

    match (g0, g1) {
        (Some(f0), Some(f1)) if f0 < f1 => (start + i1 - 1, f1),
        (Some(f0), _) => (start + i0 - 1, f0),
        (None, _) => unreachable!(),
    }
}

#[test]
fn extremum_count() {
    let mut fl = 1;
    let mut fr = 2;
    for i in 1..=15 {
        for n in fl..fr {
            for k in 0..n {
                let mut count = 0;
                let f = |i| {
                    count += 1;
                    -(i as i32 - k as i32).abs()
                };
                let res = extremum(0..n, f);
                assert_eq!(res, (k, 0));
                assert!(count <= i);
            }
        }
        let tmp = fl + fr;
        fl = std::mem::replace(&mut fr, tmp);
    }
}
