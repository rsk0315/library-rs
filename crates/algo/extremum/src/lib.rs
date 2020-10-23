//! 三分探索。

use std::convert::From;
use std::ops::{Add, Range, Sub};

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
/// コード長はやや長くなるものの、単純な例での実測では三等分のものよりわずかに高速だった。
///
/// # Complexity
/// 関数 $f$ の呼び出しを $\\log\_{\\varphi}(r-l)+O(1)$ 回行う。
/// また、関数値どうしの比較を $2\\cdot\\log\_{\\varphi}(r-l)+O(1)$ 回行う。
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
pub fn extremum<I, T, F>(Range { start, end }: Range<I>, f: F) -> (I, T)
where
    I: Add<Output = I> + Sub<Output = I> + From<u8> + Copy + Ord,
    F: Fn(I) -> T,
    T: Ord,
{
    let mut i0: I = 0.into();
    let mut i1: I = 1.into();
    let n = end - start;
    while i0 + i1 < n {
        let tmp = i0 + i1;
        i0 = std::mem::replace(&mut i1, tmp);
    }
    let g = |i| if i < n { Some(f(start + i)) } else { None };
    let mut d = i0;
    let mut g0 = g(i0);
    let mut g1 = g(i1);
    while d > 0.into() {
        match (g0, g1) {
            (Some(f0), Some(f1)) if f0 < f1 => {
                let tmp = i0 + d;
                i0 = std::mem::replace(&mut i1, tmp);
                g0 = Some(f1);
                g1 = g(i1);
            }
            (f0, _) => {
                let tmp = i1 - d;
                i1 = std::mem::replace(&mut i0, tmp);
                g1 = f0;
                g0 = g(i0);
            }
        }
        d = d + i0 - i1;
    }
    match (g0, g1) {
        (Some(f0), Some(f1)) if f0 < f1 => (start + i1, f1),
        (Some(f0), _) => (start + i0, f0),
        (None, _) => unreachable!(),
    }
}
