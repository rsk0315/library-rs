//! 連分数展開。

/// 連分数展開。
///
/// $[a\_0, a\_1, \\dots, a\_{i-1}]$ の連分数展開。
/// $a\_\\bullet$ を生成するイテレータから
/// $$
/// a\_0 + \\frac{1}{a\_1 + \\frac{1}{\\cdots + \\frac{1}{a\_{i-1} + \\frac{1}{a\_\\bullet$}}}}
/// $$
/// を生成するイテレータを作る。
///
/// # Examples
/// ```
/// use nekolib::math::continued_fraction;
///
/// let sqrt2 = std::iter::once(1).chain(std::iter::repeat(2));
/// let mut it = continued_fraction(sqrt2);
/// assert_eq!(it.next(), Some((1, 1)));
/// assert_eq!(it.next(), Some((3, 2)));
/// assert_eq!(it.next(), Some((7, 5)));
/// assert_eq!(it.next(), Some((17, 12)));
/// assert_eq!(it.next(), Some((41, 29)));
/// ```
pub fn continued_fraction(
    mut a: impl Iterator<Item = i128>,
) -> impl Iterator<Item = (i128, i128)> {
    let a0 = a.next().unwrap();
    std::iter::successors(
        Some(((1, a0), (0, 1))),
        move |&((nx, ny), (dx, dy))| {
            let ai = a.next().unwrap();
            Some(((ny, nx + ai * ny), (dy, dx + ai * dy)))
        },
    )
    .map(|((_, ny), (_, dy))| (ny, dy))
}
