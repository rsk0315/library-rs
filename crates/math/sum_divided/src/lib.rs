//! $ \\sum\_{i=0}\^{n-1} \\left\\lfloor\\frac{ai+b}{m}\\right\\rfloor. $

/// $ \\sum\_{i=0}\^{n-1} \\left\\lfloor\\frac{ai+b}{m}\\right\\rfloor. $
///
/// # Requirements
/// - $n \\ge 0$
/// - $m \\gt 0$
///
/// # Idea
/// あとで書く。
///
/// ## See also
/// - <https://rsk0315.hatenablog.com/entry/2020/12/13/231307>
/// - <https://atcoder.jp/contests/practice2/editorial/579>
///
/// # Examples
/// ```
/// use nekolib::math::sum_divided;
///
/// assert_eq!(sum_divided(4, 10, 6, 3), 3);
/// assert_eq!(sum_divided(6, 5, 4, 3), 13);
/// assert_eq!(sum_divided(1, 1, 0, 0), 0);
/// assert_eq!(sum_divided(31415, 92653, 58979, 32384), 314095480);
/// assert_eq!(
///     sum_divided(1000000000, 1000000000, 999999999, 999999999),
///     499999999500000000
/// );
/// assert_eq!(sum_divided(14, 23, -7, -39), -58);
/// ```
pub fn sum_divided(n: i128, m: i128, a: i128, b: i128) -> i128 {
    assert!(m > 0);
    assert!(n > 0);
    let mut res = 0;
    let a = if a < 0 {
        let a2 = a.rem_euclid(m);
        res -= n * (n - 1) / 2 * ((a2 - a) / m);
        a2
    } else {
        a
    } as u128;
    let b = if b < 0 {
        let b2 = b.rem_euclid(m);
        res -= n * ((b2 - b) / m);
        b2
    } else {
        b
    } as u128;
    res + sum_divided_unsigned(n as u128, m as u128, a, b) as i128
}

fn sum_divided_unsigned(
    mut n: u128,
    mut m: u128,
    mut a: u128,
    mut b: u128,
) -> u128 {
    let mut res = 0;
    loop {
        if a >= m {
            res += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            res += n * (b / m);
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    res
}
