//! 二分探索。

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
/// `buf.len()` を $n$ として、$\\log_2(n)+O(1)$ 回の `pred` の呼び出しを行う。
///
/// # Examples
/// ```
/// use nekolib::algo::bisect;
///
/// assert_eq!(bisect(&[2, 4, 7], |&x| x < 4), 1);
/// assert_eq!(bisect(&[2, 4, 7], |&x| x <= 4), 2);
/// assert_eq!(bisect(&[2, 4, 7], |&_| false), 0);
/// assert_eq!(bisect(&[2, 4, 7], |&_| true), 3);
/// ```
pub fn bisect<T, F: Fn(&T) -> bool>(buf: &[T], pred: F) -> usize {
    if buf.is_empty() || !pred(&buf[0]) {
        return 0;
    }

    let mut ok = 0;
    let mut bad = buf.len();
    while bad - ok > 1 {
        let mid = ok + (bad - ok) / 2;
        match pred(&buf[mid]) {
            true => ok = mid,
            false => bad = mid,
        }
    }
    bad
}
