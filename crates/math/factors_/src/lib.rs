//! 素因数分解。

use std::collections::BTreeMap;

/// 素因数分解。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::factors;
///
/// let fac: Vec<_> = factors(735134400).collect();
/// assert_eq!(fac, [(2, 6), (3, 3), (5, 2), (7, 1), (11, 1), (13, 1), (17, 1)]);
/// ```
pub fn factors(
    mut n: u64,
) -> impl Iterator<Item = (u64, u32)> + DoubleEndedIterator {
    let mut res = BTreeMap::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let mut e = 0;
            while n % i == 0 {
                n /= i;
                e += 1;
            }
            res.insert(i, e);
        }
        i += 1;
    }
    if n > 1 {
        res.insert(n, 1);
    }
    res.into_iter()
}
