//! スライスの最小値・最大値を求める。

use std::cmp::Ordering::{self, Equal, Greater, Less};

/// スライスの最小値および最大値を求める。
///
/// 該当する要素が複数個あった場合、最小値は最左のもの、最大値は最右のものが選ばれる。
///
/// # Suggestions
/// 最小値・最大値の添字ではなく最小値・最大値自体を返すようになっている。
/// 添字が欲しい場合は [`minmax_by_key`] を利用するのがよい？ あるいは設計を変える？
///
/// # Examples
/// ```
/// use nekolib::algo::minmax;
///
/// assert_eq!(minmax(&[3, 2, 4, 1, 2, 0, 6]), Some((&0, &6)));
/// assert_eq!(minmax(&[]), None);
/// ```
pub fn minmax<T: Ord>(buf: &[T]) -> Option<(&T, &T)> {
    minmax_by(buf, |x: &T, y: &T| x.cmp(y))
}

/// キー `key` におけるスライスの最小値および最大値を求める。
///
/// 該当する要素が複数個あった場合、最小値は最左のもの、最大値は最右のものが選ばれる。
///
/// # Examples
/// ```
/// use nekolib::algo::minmax_by_key;
///
/// let buf =
///     vec![3, 5, 0, 1, 2, 0, 5]into_iter().enumerate().collect();
///
/// assert_eq!(minmax_by_key(&buf, |&(_, x)| x), Some((&(2, 0), &(6, 5))));
///
/// let buf: Vec<i32> = vec![];
/// assert_eq!(minmax_by_key(&buf, |&x| x), None);
/// ```
pub fn minmax_by_key<T, K, U>(buf: &[T], key: K) -> Option<(&T, &T)>
where
    K: Fn(&T) -> U,
    U: Ord,
{
    minmax_by(buf, |x: &T, y: &T| key(&x).cmp(&key(&y)))
}

/// 比較関数 `compare` におけるスライスの最小値および最大値を求める。
///
/// 該当する要素が複数個あった場合、最小値は最左のもの、最大値は最右のものが選ばれる。
///
/// # Examples
/// ```
/// use nekolib::algo::minmax_by;
///
/// let buf: Vec<_> =
///     vec![3, 9, 0, 1, 2, 0, 9].into_iter().enumerate().collect();
/// let rev = |&(_, x): &(usize, i32), &(_, y): &(usize, i32)| y.cmp(&x);
/// assert_eq!(minmax_by(&buf, rev), Some((&(1, 9), &(5, 0))));
///
/// let buf: Vec<(usize, i32)> = vec![];
/// assert_eq!(minmax_by(&buf, rev), None);
/// ```
pub fn minmax_by<T, F: Fn(&T, &T) -> Ordering>(
    buf: &[T],
    compare: F,
) -> Option<(&T, &T)> {
    if buf.is_empty() {
        return None;
    }
    if buf.len() == 1 {
        return Some((&buf[0], &buf[0]));
    }
    let (mut min, mut max) = match compare(&buf[0], &buf[1]) {
        Less => (&buf[0], &buf[1]),
        Equal => (&buf[0], &buf[0]),
        Greater => (&buf[1], &buf[0]),
    };
    for i in (2..buf.len()).step_by(2) {
        let (first, second) = match (buf.get(i), buf.get(i + 1)) {
            (Some(f), Some(s)) => (f, s),
            (Some(f), None) => (f, f),
            (None, _) => unreachable!(),
        };
        let (min_i, max_i) = match compare(first, second) {
            Less => (first, second),
            Equal => (first, first),
            Greater => (second, first),
        };
        if compare(min_i, min) == Less {
            min = min_i;
        }
        if compare(max_i, max) != Less {
            max = max_i;
        }
    }
    Some((min, max))
}
