//! 平方根の連分数展開。

use super::sqrt;

use sqrt::Sqrt;

/// 平方根の連分数展開。
///
/// $\\sqrt{n}$ の連分数展開を $[a\_0; a\_1, a\_2, \\dots]$ とする。
/// このとき、$a\_\\bullet$ を生成するイテレータを返す。
///
/// # Examples
/// ```
/// use nekolib::math::sqrt_fraction;
///
/// let a: Vec<_> = sqrt_fraction(3).take(30).map(|x| x as f64).collect();
/// let r = a.into_iter().rev().reduce(|a1, a0| a0 + 1.0 / a1).unwrap();
///
/// assert!((r - 3.0_f64.sqrt()).abs() < 1.0e-16);
/// ```
pub fn sqrt_fraction(n: i128) -> impl Iterator<Item = i128> {
    let (r, next) = sqrt_fraction_fn(n);
    std::iter::successors(Some((r, r, 1)), move |&(_, b, c)| Some(next(b, c)))
        .map(|(a, ..)| a)
}

/// 平方根の連分数展開。
///
/// $\\sqrt{n}$ の連分数展開の $i$ step 目が次のように表されるとする。
/// $$ \\sqrt{n} = a\_0 +
/// \\frac{1}{\\dots\\,+\\frac{1}{a\_{i-1}+\\frac{\\sqrt{n}-b\_{i-1}}{c\_{i-1}}}}.
/// $$
///
/// $a\_0$ と、関数 $f: (b\_i, c\_i)\\mapsto (a\_{i+1}, b\_{i+1}, c\_{i+1})$
/// を返す。
/// ただし、$(a\_0, b\_0, c\_0)
/// = (\\lfloor\\sqrt{n}\\rfloor, \\lfloor\\sqrt{n}\\rfloor, 1)$ である。
///
/// 実際の連分数展開が欲しいときは [`sqrt_fraction`] を用いればよい。
/// 周期検出などをしたいときは $(b\_\\bullet, c\_\\bullet)$ が必要になる。
///
/// # Derivation
/// `todo!()`
///
/// # Examples
/// ```
/// use nekolib::math::sqrt_fraction_fn;
///
/// let (a0, next) = sqrt_fraction_fn(3);
/// assert_eq!(a0, 1);
/// let (a1, b1, c1) = next(a0, 1);
/// let (a2, b2, c2) = next(b1, c1);
/// let (a3, b3, c3) = next(b2, c2);
///
/// assert_eq!((a1, b1, c1), (a3, b3, c3));  // sqrt(3) has period 2
/// assert_eq!([a0, a1, a2], [1, 1, 2]);  // sqrt(3) = [1; (1, 2)]
/// ```
pub fn sqrt_fraction_fn(
    n: i128,
) -> (i128, impl Fn(i128, i128) -> (i128, i128, i128)) {
    let r = n.sqrt();
    let next = move |b, c| {
        assert_eq!((n - b * b) % c, 0);
        let c_ = (n - b * b) / c;
        let a_ = (r + b) / c_;
        let b_ = a_ * c_ - b;
        (a_, b_, c_)
    };
    (r, next)
}
