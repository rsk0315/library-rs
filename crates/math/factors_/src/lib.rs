//! 素因数分解。

/// 素因数分解。
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
/// assert_eq!(factors(n).map(|(p, e)| p.pow(e)).product::<u64>(), n);
/// ```
pub fn factors(mut n: u64) -> impl Iterator<Item = (u64, u32)> {
    (2..)
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
