//! 約数列挙。

/// 約数列挙。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::divisors;
///
/// let div: Vec<_> = divisors(60).collect();
/// assert_eq!(div, [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
/// ```
pub fn divisors(n: u64) -> impl Iterator<Item = u64> + DoubleEndedIterator {
    let mut former = vec![];
    let mut latter = vec![];
    for i in (1..=n).take_while(|&i| i * i <= n).filter(|&i| n % i == 0) {
        former.push(i);
        if i * i < n {
            latter.push(n / i);
        }
    }
    former.into_iter().chain(latter.into_iter().rev())
}
