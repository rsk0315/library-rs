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
/// # Complexity
/// [`minmax_by`] における `compare` の呼び出し回数と同じだけ、要素間の比較を行う。
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
/// # Complexity
/// [`minmax_by`] における `compare` の呼び出し回数と同じだけ、要素間の比較を行う。
/// また、`key` の呼び出しをその $2$ 倍の回数だけ行う。
///
/// # Implementation notes
/// 実装を `minmax_by` に丸投げしているので `key` を $3n$ 回程度呼び出しうるが、
/// 適切に実装することで $n$ 回に抑えられるはず。
///
/// `key` のコストが大きい場合は予め別の配列を作る方がよさそう。
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
pub fn minmax_by_key<T, K, U>(buf: &[T], mut key: K) -> Option<(&T, &T)>
where
    K: FnMut(&T) -> U,
    U: Ord,
{
    minmax_by(buf, |x: &T, y: &T| key(&x).cmp(&key(&y)))
}

/// 比較関数 `compare` におけるスライスの最小値および最大値を求める。
///
/// 該当する要素が複数個あった場合、最小値は最左のもの、最大値は最右のものが選ばれる。
///
/// # Complexity
/// 要素数を $n$ として、`compare` の呼び出しを高々
/// $\\max\\{0, \\lfloor\\frac{n}{2}\\rfloor + 2\\cdot\\lfloor{n-1}{2}\\rfloor\\} \\le 1.5n$
/// 回行う。
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
pub fn minmax_by<T, F: FnMut(&T, &T) -> Ordering>(
    buf: &[T],
    mut compare: F,
) -> Option<(&T, &T)> {
    if buf.is_empty() {
        return None;
    }
    if buf.len() == 1 {
        return Some((&buf[0], &buf[0]));
    }
    let (mut min, mut max) = match compare(&buf[0], &buf[1]) {
        Less | Equal => (&buf[0], &buf[1]),
        Greater => (&buf[1], &buf[0]),
    };
    for i in (2..buf.len()).step_by(2) {
        let (min_i, max_i) = match (buf.get(i), buf.get(i + 1)) {
            (Some(f), None) => (f, f),
            (Some(f), Some(s)) => match compare(f, s) {
                Less | Equal => (f, s),
                Greater => (s, f),
            },
            (None, _) => unreachable!(),
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

#[test]
fn test() {
    use std::fmt::Debug;
    fn test_inner<T: Debug + Eq + Ord>(
        expected: Option<(&(usize, &T), &(usize, &T))>,
        buf: &[T],
    ) {
        let mut cmped = 0;
        let counted_cmp = |x: &(usize, &T), y: &(usize, &T)| {
            cmped += 1;
            x.1.cmp(y.1)
        };
        let buf: Vec<_> = buf.iter().enumerate().collect();
        let n = buf.len();
        assert_eq!(minmax_by(&buf, counted_cmp), expected);
        assert!(cmped <= n / 2 + (n - 1) / 2 * 2);
    }

    test_inner(Some((&(0, &0), &(0, &0))), &[0]);

    test_inner(Some((&(0, &0), &(1, &0))), &[0, 0]);
    test_inner(Some((&(0, &0), &(1, &10))), &[0, 10]);
    test_inner(Some((&(1, &0), &(0, &10))), &[10, 0]);

    test_inner(Some((&(0, &0), &(2, &0))), &[0, 0, 0]);
    test_inner(Some((&(0, &0), &(2, &20))), &[0, 10, 20]);
    test_inner(Some((&(0, &0), &(2, &10))), &[0, 10, 10]);
    test_inner(Some((&(0, &10), &(1, &20))), &[10, 20, 10]);

    test_inner(Some((&(0, &0), &(3, &0))), &[0, 0, 0, 0]);
    test_inner(Some((&(0, &0), &(3, &10))), &[0, 10, 0, 10]);
    test_inner(Some((&(1, &0), &(2, &10))), &[10, 0, 10, 0]);
    test_inner(Some((&(0, &0), &(3, &10))), &[0, 0, 10, 10]);
    test_inner(Some((&(2, &0), &(1, &10))), &[10, 10, 0, 0]);
}
