//! 素因数分解。

/// 素因数分解（指数）。
///
/// $n = \\prod\_{p:\\text{ prime}} p\_i^{e\_i}$ に対して、各 $(p\_i, e\_i)$ を $p\_i$
/// の昇順に返す。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::factors;
///
/// let n = 735134400;
/// let fac: Vec<_> = factors(n).collect();
/// assert_eq!(fac, [(2, 6), (3, 3), (5, 2), (7, 1), (11, 1), (13, 1), (17, 1)]);
/// assert_eq!(fac.iter().map(|&(p, e)| p.pow(e)).product::<u64>(), n);
///
/// assert_eq!(factors(1).next(), None);
/// ```
pub fn factors(mut n: u64) -> impl Iterator<Item = (u64, u32)> {
    std::iter::once(2)
        .chain((3..).step_by(2))
        .filter_map(move |i| {
            if n <= 1 {
                Some(None)
            } else if i * i > n {
                Some(Some((std::mem::take(&mut n), 1)))
            } else if n % i != 0 {
                None
            } else {
                let e = (0..).take_while(|_| n % i == 0 && (|_| true)(n /= i));
                Some(Some((i, e.count() as u32)))
            }
        })
        .take_while(Option::is_some)
        .map(Option::unwrap)
}

/// 素因数分解（重複）。
///
/// $n = \\prod\_{p:\\text{ prime}} p\_i^{e\_i}$ に対して、各 $p\_i$ を $e\_i$ 個、$p\_i$
/// の昇順に返す。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::factors_dup;
///
/// let n = 735134400;
/// let fac: Vec<_> = factors_dup(n).collect();
/// assert_eq!(fac, [2, 2, 2, 2, 2, 2, 3, 3, 3, 5, 5, 7, 11, 13, 17]);
/// assert_eq!(fac.iter().product::<u64>(), n);
///
/// assert_eq!(
///     factors_dup(2_u64.pow(5) * 3_u64.pow(5)).product::<u64>(),
///     6_u64.pow(5)
/// );
///
/// assert_eq!(factors_dup(1).next(), None);
/// ```
pub fn factors_dup(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some((2, n, false)), |&(i, n, _)| {
        if n == 1 {
            None
        } else if i * i > n {
            Some((n, 1, true))
        } else if n % i == 0 {
            Some((i, n / i, true))
        } else {
            Some((if i == 2 { 3 } else { i + 2 }, n, false))
        }
    })
    .filter_map(|(i, _, c)| if c { Some(i) } else { None })
}
