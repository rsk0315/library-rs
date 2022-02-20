//! 二分探索。

use std::ops::Range;

/// 二分探索で境界を探す。
///
/// `pred(&buf[i])` が `false` となる最小の `i` を返す。
/// ただし `i < buf.len()` なる全ての `i` で `true` の場合は `buf.len()` を返す。
/// 先頭から `i` 個の要素が条件を満たすと考えるのがよい。
///
/// # Requirements
/// `pred(&buf[i])` が `false` となる `i` が存在するとき、`i < j` なる全ての `j` で
/// `pred(&buf[j])` が `false` となる。
///
/// # Complexity
/// `buf.len()` を $n$ として、高々 $\\lceil\\log_2(n+1)\\rceil$ 回の `pred` の呼び出しを行う。
///
/// # Examples
/// ```
/// use nekolib::algo::bisect_slice;
///
/// assert_eq!(bisect_slice(&[2, 4, 7], |&x| x < 4), 1);
/// assert_eq!(bisect_slice(&[2, 4, 7], |&x| x <= 4), 2);
/// assert_eq!(bisect_slice(&[2, 4, 7], |&_| false), 0);
/// assert_eq!(bisect_slice(&[2, 4, 7], |&_| true), 3);
/// ```
pub fn bisect_slice<T>(buf: &[T], mut pred: impl FnMut(&T) -> bool) -> usize {
    bisect(0..buf.len(), |i| pred(&buf[i]))
}

/// 二分探索で境界を探す。
///
/// `pred(i)` が `false` となる最小の `i` を返す。
/// ただし `start..end` 内の全ての `i` で `true` の場合は `end` を返す。
///
/// # Requirements
/// `pred(i)` が `false` となる `i` が存在するとき、`i < j` なる全ての `j` で
/// `pred(j)` が `false` となる。
///
/// # Complexity
/// 区間の長さを $n$ として、高々 $\\lceil\\log_2(n+1)\\rceil$ 回の `pred` の呼び出しを行う。
///
/// # Suggestions
/// 範囲の型を `PrimInt` なり `Ord` なりにしたい気もする。区間長と中間値の取得をよしなにできると助かる。
///
/// # Examples
/// ```
/// use nekolib::algo::bisect;
///
/// let floor_sqrt = |i| if i <= 1 { i } else {
///     bisect(0..i, |j| match (j + 1).overflowing_pow(2) {
///         (x, false) if x <= i => true,
///         _ => false
///     })
/// };
/// assert_eq!(floor_sqrt(8), 2);
/// assert_eq!(floor_sqrt(9), 3);
/// assert_eq!(floor_sqrt(10), 3);
/// assert_eq!(floor_sqrt(1 << 60), 1 << 30);
/// ```
pub fn bisect(
    Range { start, end }: Range<usize>,
    mut pred: impl FnMut(usize) -> bool,
) -> usize {
    if start == end {
        return start;
    }
    let mut ok = start;
    let mut bad = end;
    while bad - ok > 1 {
        let mid = ok + (bad - ok) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            bad = mid;
        }
    }
    if ok == start && !pred(start) {
        start
    } else {
        bad
    }
}

#[test]
fn bisect_count() {
    for n in 0..=128 {
        for k in 0..=n {
            let mut count = 0;
            let f = |i| {
                count += 1;
                i < k
            };
            let res = bisect(0..n, f);
            assert!(count <= (n + 1).next_power_of_two().trailing_zeros());
            assert_eq!(res, k);
        }
    }
}
