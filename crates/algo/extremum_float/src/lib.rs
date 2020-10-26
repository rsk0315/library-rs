//! 三分探索（実数）。

use std::ops::Range;

/// 三分探索で極値を探す。
///
/// # Requirements
/// 凸である。
///
/// # Implementation notes
/// 黄金比を用いて分割する実装のため、関数値を使い回すことができる。
///
/// 関数 $f$ の呼び出し回数を、区間を三等分する素直な実装と比較する。
///
/// `todo!()`
///
/// # Parameters
/// `todo!()`
///
/// # Complexity
/// `todo!()`
///
/// # Suggestions
/// `f64` に限らずジェネリックにするべき？ `NonNan` とかを渡したいときもありそう。
/// 関数の返り値も `T: PartialOrd` にする？
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
/// let (x, y) = extremum_float(xl..xr, eps, f);
/// let y = -y;
///
/// let e = std::f64::consts::E;
/// assert!(((1.0 / e) - x).abs() < eps);
/// assert!((e.powf(-1.0 / e) - y).abs() < eps);
/// ```
pub fn extremum_float<F>(range: Range<f64>, eps: f64, f: F) -> (f64, f64)
where
    F: Fn(f64) -> f64,
{
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let phi_p1 = phi + 1.0;

    let Range {
        start: mut xl,
        end: mut xr,
    } = range;

    let iter =
        unsafe { ((xr - xl) / eps).log(phi).to_int_unchecked::<u32>() } + 1;

    let mut xml = (phi * xl + xr) / phi_p1;
    let mut xmr = (xl + phi * xr) / phi_p1;
    let mut yml = f(xml);
    let mut ymr = f(xmr);

    for i in 0..iter {
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
