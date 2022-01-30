//! 添字の順序。

use std::cmp::Ordering;

/// 添字の順序。
///
/// See [`index_order_by`].
///
/// [`index_order_by`]: fn.index_order_by.html
///
/// # Examples
/// ```
/// use std::cmp::Reverse;
///
/// use nekolib::algo::index_order_by_key;
///
/// let a = [0, 2, 1, 4, 5, 1, 3];
/// let key = |(i, &ai): (usize, &i32)| (ai, Reverse(i));
/// assert_eq!(index_order_by_key(&a, key), [0, 5, 2, 1, 6, 3, 4]);
/// ```
pub fn index_order_by_key<T, K: Ord>(
    buf: &[T],
    mut key: impl FnMut((usize, &T)) -> K,
) -> Vec<usize> {
    index_order_by(buf, |l, r| key(l).cmp(&key(r)))
}

/// 添字の順序。
///
/// 次で定義される $b = (b\_0, b\_1, \\dots, b\_{|a|-1})$ を返す。
/// - $b\_i$ は相異なる
/// - $b\_i \\in [0, |a|)$
/// - $a\_{b\_0} \\preceq a\_{b\_1} \\preceq \\cdots \\preceq a\_{b\_{|a|-1}}$
///
/// ただし $\\preceq$ は `compare` による順序とする。
///
/// # See also
///
/// [`index_order_by_key`].
///
/// [`index_order_by_key`]: fn.index_order_by_key.html
///
/// # Examples
/// ```
/// use std::cmp::Ordering;
///
/// use nekolib::algo::index_order_by;
///
/// // See <https://ngtkana.hatenablog.com/entry/2021/11/13/202103>
/// let argcmp = |[x0, y0]: [i64; 2], [x1, y1]: [i64; 2]| {
///     (([y0, x0] < [0; 2]).cmp(&([y1, x1] < [0; 2])))
///         .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
/// };
///
/// let a = [[1, 1], [1, -1], [-1, 0], [0, 1], [1, 0], [0, -1], [-1, 1], [-1, -1]];
/// let compare =
///     |(_, &al): (usize, &[i64; 2]), (_, &ar): (usize, &[i64; 2])| argcmp(al, ar);
///
/// // [6] [3] [0]
/// // [2]  O  [4]
/// // [7] [5] [1]
/// assert_eq!(index_order_by(&a, compare), [4, 0, 3, 6, 2, 7, 5, 1]);
/// ```
pub fn index_order_by<T>(
    buf: &[T],
    mut compare: impl FnMut((usize, &T), (usize, &T)) -> Ordering,
) -> Vec<usize> {
    let n = buf.len();
    let mut res: Vec<_> = (0..n).collect();
    res.sort_unstable_by(|&l, &r| compare((l, &buf[l]), (r, &buf[r])));
    res
}
