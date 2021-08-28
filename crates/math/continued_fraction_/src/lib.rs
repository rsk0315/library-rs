//! 連分数展開。

/// 連分数展開。
///
/// $[a\_0, a\_1, \\dots, a\_{i-1}]$ の連分数展開。
/// $a\_\\bullet$ を生成するイテレータから
/// $$
/// a\_0 + \\frac{1}{a\_1 + \\frac{1}{\\cdots\\, + \\frac{1}{a\_{\\bullet}}}}
/// $$
/// を生成するイテレータを作る。
///
/// # Implementation notes
/// [`std::iter::successors`](https://doc.rust-lang.org/std/iter/fn.successors.html)
/// によって次の項が計算されるタイミングの関係で、
/// 実際には必要ない値の計算のせいでオーバーフローし、panic するのを避けるため、
/// 同等の処理を `map` 中に再度書いている。
///
/// Pell 方程式を解く際、解は [`i128`](https://doc.rust-lang.org/std/primitive.i128.html)
/// に収まるが上記の問題で panic したのでこうなった。
/// 実際には release build では問題なく動くことが予想されるので無視してもいい気もするが...
///
/// # Examples
/// ```
/// use nekolib::math::continued_fraction;
///
/// let frac_exp = [1, 2];
/// let sqrt3 = std::iter::once(1).chain(frac_exp.iter().copied().cycle());
/// let mut it = continued_fraction(sqrt3);
/// assert_eq!(it.next(), Some((1, 1)));
/// assert_eq!(it.next(), Some((2, 1)));
/// assert_eq!(it.next(), Some((5, 3)));
/// assert_eq!(it.next(), Some((7, 4)));
/// assert_eq!(it.next(), Some((19, 11)));
/// assert_eq!(it.next(), Some((26, 15)));
///
/// let (x, y) = it.nth(24).unwrap();
/// assert!(((x as f64 / y as f64) - 3.0_f64.sqrt()).abs() < 1.0e-16);
/// ```
pub fn continued_fraction(
    mut a: impl Iterator<Item = i128>,
) -> impl Iterator<Item = (i128, i128)> {
    let a0 = a.next().unwrap();
    let frac = std::iter::successors(
        Some(((1, a0), (0, 1), a.next().unwrap())),
        move |&((nx, ny), (dx, dy), ai)| {
            Some(((ny, nx + ai * ny), (dy, dx + ai * dy), a.next().unwrap()))
        },
    )
    .map(|((nx, ny), (dx, dy), ai)| (nx + ai * ny, dx + ai * dy));
    std::iter::once((a0, 1)).chain(frac)
}
