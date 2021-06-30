//! Boyer--Moore's majority vote algorithm。

/// Boyer--Moore's majority vote algorithm。
///
/// 過半数の出現数を持つ要素があれば、それを返す。
///
/// # Idea
/// `todo!()`
///
/// # Complexity
/// $O(n)$ time.
///
/// # Examples
/// ```
/// use nekolib::algo::majority;
///
/// assert_eq!(majority(&[1, 1, 3, 2, 1]), Some(&1));
/// assert_eq!(majority(&[9]), Some(&9));
/// assert_eq!(majority(&[6, 7]), None);
/// assert_eq!(majority::<i32>(&[]), None);
/// ```
pub fn majority<T: Eq>(buf: &[T]) -> Option<&T> {
    if buf.is_empty() {
        return None;
    }
    let mut maj = &buf[0];
    let mut vote = 1;
    let n = buf.len();
    for x in buf.iter().skip(1) {
        if maj == x {
            vote += 1;
        } else if vote == 0 {
            maj = x;
            vote = 1;
        } else {
            vote -= 1;
        }
    }
    let mut vote = 0;
    let mut first = 0;
    for (i, x) in buf.iter().enumerate().rev() {
        if maj == x {
            vote += 1;
            first = i;
        }
    }
    if vote > n - vote {
        Some(&buf[first])
    } else {
        None
    }
}
