//! 三分探索（実数）。

use std::ops::RangeInclusive;

/// 三分探索で極値を探す。
///
/// 関数 $f$ の $[x\_l, x\_r]$ における極大値を $x\^\\ast$ として、
/// $|x-x\^\\ast| \\le \\varepsilon$ なる $x$ を求め、$(x, f(x))$ を返す。
///
/// # Requirements
/// $f$ は凸である。
///
/// # Notes
/// 黄金比を用いて分割する実装のため、関数値を使い回すことができる。
///
/// # Complexity
/// `f` の呼び出しを $\\log\_{\\varphi}(\\frac{x\_r-x\_l}{\\varepsilon}) + 1$ 回行う。
///
/// # Suggestions
/// `f64` に限らずジェネリックにするべき？ 関数の返り値も `T: PartialOrd` にする？
///
/// # Examples
/// $f(x) = x\^x$ の最小値を求める。
///
/// $x = 1/e$ のとき、最小値 $e\^{-1/e}$ をとる。
/// cf. [Wolfram|Alpha](https://www.wolframalpha.com/input/?i=y+%3D+x**x)
/// ```
/// use nekolib::algo::extremum_float;
///
/// let p = 3.0_f64;
/// let f = |x: f64| -x.powf(x);
///
/// let xl = 0.0;
/// let xr = 140.0;
/// let eps = 1.0e-8;
/// let (x, y) = extremum_float(xl..=xr, eps, f);
/// let y = -y;
///
/// let e = std::f64::consts::E;
/// assert!(((1.0 / e) - x).abs() < eps);
/// assert!((e.powf(-1.0 / e) - y).abs() < eps);
/// ```
pub fn extremum_float(
    range: RangeInclusive<f64>,
    eps: f64,
    mut f: impl FnMut(f64) -> f64,
) -> (f64, f64) {
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let phi_p1 = phi + 1.0;

    let &(mut xl) = range.start();
    let &(mut xr) = range.end();

    let iter = ((xr - xl) / eps).log(phi) as u32 + 1;

    let mut xml = (phi * xl + xr) / phi_p1;
    let mut xmr = (xl + phi * xr) / phi_p1;
    let mut yml = f(xml);
    let mut ymr = f(xmr);

    for _ in 0..iter {
        if yml > ymr {
            xr = std::mem::replace(&mut xmr, xml);
            ymr = yml;
            xml = (phi * xl + xr) / phi_p1;
            yml = f(xml);
        } else {
            xl = std::mem::replace(&mut xml, xmr);
            yml = ymr;
            xmr = (xl + phi * xr) / phi_p1;
            ymr = f(xmr);
        }
    }
    (xml, yml)
}
