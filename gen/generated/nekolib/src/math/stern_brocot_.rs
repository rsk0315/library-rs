//! Stern--Brocot tree

/// Stern--Brocot tree
///
/// $a/b = 0/1=0$ と $c/d = 1/0=\\infty$ で初期化し、その mediant
/// $x/y = (a+b)/(c+d)$
/// を探索する。
///
/// $x/y$ が所望の条件 `ok` を満たすならそれを [`Ok`] として返す。
/// 条件 `large` を満たすなら $c/d\\gets x/y$、そうでなければ $a/b\\gets x/y$
/// として探索を進める。
/// 分母が $n$ を超えても `ok` を満たさなければ、$((a, b), (c, d))$ を
/// [`Err`] として返す。
///
/// [`Ok`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Ok
/// [`Err`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html#variant.Err
///
/// # Examples
/// ```
/// use nekolib::math::stern_brocot;
///
/// let (num, den) =
///     stern_brocot(10_i128.pow(18), |n, d| n * n > 3 * d * d, |_, _| false)
///     .err().unwrap().0;
///
/// let sqrt3 = num as f64 / den as f64;
/// assert!((sqrt3 - 3.0_f64.sqrt()).abs() < 1.0e-16);
/// assert_eq!((num, den), (734231055024833855, 423908497265970753));
/// ```
pub fn stern_brocot(
    n: i128,
    mut large: impl FnMut(i128, i128) -> bool,
    mut ok: impl FnMut(i128, i128) -> bool,
) -> Result<(i128, i128), ((i128, i128), (i128, i128))> {
    let (mut a, mut b) = (0, 1);
    let (mut c, mut d) = (1, 0);
    loop {
        let x = a + c;
        let y = b + d;
        if y > n {
            return Err(((a, b), (c, d)));
        }
        if ok(x, y) {
            return Ok((x, y));
        }
        if large(x, y) {
            c = x;
            d = y;
        } else {
            a = x;
            b = y;
        }
    }
}
