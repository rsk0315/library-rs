//! 順列。

/// 辞書順で次の順列の生成。
///
/// # Idea
/// `todo!()`
///
/// # References
/// - <https://stackoverflow.com/questions/11483060>
///
/// # Examples
/// ```
/// use nekolib::algo::next_permutation;
///
/// let mut a = vec![1, 3, 2];
/// assert!(next_permutation(&mut a));
/// assert_eq!(a, [2, 1, 3]);
///
/// // last one
/// let mut a = vec![3, 2];
/// assert!(!next_permutation(&mut a));
/// assert_eq!(a, [2, 3]);
///
/// // empty one
/// let mut a = Vec::<()>::new();
/// assert!(!next_permutation(&mut a));
///
/// // duplicated one
/// let mut a = vec![1, 3, 2, 2, 3];
/// next_permutation(&mut a);
/// assert_eq!(a, [1, 3, 2, 3, 2]);
pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n <= 1 {
        return false;
    }

    let mut i = n - 1;
    loop {
        let j = i;
        i -= 1;
        if a[i] < a[j] {
            let k = (0..n).rev().find(|&k| a[i] < a[k]).unwrap();
            a.swap(i, k);
            a[j..].reverse();
            return true;
        }
        if i == 0 {
            a.reverse();
            return false;
        }
    }
}
