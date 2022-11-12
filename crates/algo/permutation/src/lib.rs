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

    for i in (0..n - 1).rev() {
        if a[i] < a[i + 1] {
            let j = (0..n).rev().find(|&j| a[i] < a[j]).unwrap();
            a.swap(i, j);
            a[i + 1..].reverse();
            return true;
        }
    }

    a.reverse();
    false
}

pub fn prev_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n <= 1 {
        return false;
    }

    for i in (0..n - 1).rev() {
        if a[i] > a[i + 1] {
            let j = (0..n).rev().find(|&j| a[i] > a[j]).unwrap();
            a.swap(i, j);
            a[i + 1..].reverse();
            return true;
        }
    }

    a.reverse();
    false
}

fn next_permutation_with_count<T: Ord>(a: &mut [T], k: usize) -> bool {
    // precondition: k <= a.len(), and a[k..] is sorted
    // postcondition: a[k..] is sorted

    let n = a.len();
    for i in (0..k).rev() {
        let j = if k < n && a[i] < a[n - 1] {
            (k..).find(|&j| a[i] < a[j]).unwrap()
        } else if i + 1 < n && a[i] < a[i + 1] {
            (i..k).rev().find(|&j| a[i] < a[j]).unwrap()
        } else {
            continue;
        };
        a.swap(i, j);
        a[i + 1..k].reverse();
        a[i + 1..].rotate_right(n - k);
        return true;
    }
    false
}

fn prev_permutation_with_count<T: Ord>(a: &mut [T], k: usize) -> bool {
    // precondition: k <= a.len(), and a[k..] is reversely sorted
    // postcondition: a[k..] is reversely sorted

    let n = a.len();
    for i in (0..k).rev() {
        let j = if k < n && a[i] > a[n - 1] {
            (k..).find(|&j| a[i] > a[j]).unwrap()
        } else if i + 1 < n && a[i] > a[i + 1] {
            (i..k).rev().find(|&j| a[i] > a[j]).unwrap()
        } else {
            continue;
        };
        a.swap(i, j);
        a[i + 1..k].reverse();
        a[i + 1..].rotate_right(n - k);
        return true;
    }
    false
}

pub struct Permutations<T>(Vec<T>);

impl<T: Ord> From<Vec<T>> for Permutations<T> {
    fn from(buf: Vec<T>) -> Self { Self(buf) }
}

impl<T: Ord> Permutations<T> {
    pub fn next(&mut self) -> bool { next_permutation(&mut self.0) }
    pub fn prev(&mut self) -> bool { prev_permutation(&mut self.0) }
    pub fn peek(&self) -> &[T] { &self.0 }
}

impl<T: Ord + Clone> Permutations<T> {
    pub fn forward(self, count: usize) -> Forward<T> {
        Forward::new(self, count)
    }
    pub fn backward(self, count: usize) -> Backward<T> {
        Backward::new(self, count)
    }
}

pub struct Forward<T> {
    buf: Vec<T>,
    count: usize,
    finish: bool,
}

impl<T: Clone + Ord> Forward<T> {
    fn new(perm: Permutations<T>, count: usize) -> Self {
        let mut buf = perm.0;
        buf[count..].sort();
        Self { buf, count, finish: false }
    }
}

impl<T: Clone + Ord> Iterator for Forward<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            None
        } else {
            let tmp = self.buf[..self.count].to_vec();
            self.finish =
                !next_permutation_with_count(&mut self.buf, self.count);
            Some(tmp)
        }
    }
}

pub struct Backward<T> {
    buf: Vec<T>,
    count: usize,
    finish: bool,
}

impl<T: Clone + Ord> Backward<T> {
    fn new(perm: Permutations<T>, count: usize) -> Self {
        let mut buf = perm.0;
        buf[count..].sort_by(|x, y| y.cmp(x));
        Self { buf, count, finish: false }
    }
}

impl<T: Clone + Ord> Iterator for Backward<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finish {
            None
        } else {
            let tmp = self.buf[..self.count].to_vec();
            self.finish =
                !prev_permutation_with_count(&mut self.buf, self.count);
            Some(tmp)
        }
    }
}

#[test]
fn iter() {
    let expected = vec![
        vec![0, 1, 1, 2, 2],
        vec![0, 1, 2, 1, 2],
        vec![0, 1, 2, 2, 1],
        vec![0, 2, 1, 1, 2],
        vec![0, 2, 1, 2, 1],
        vec![0, 2, 2, 1, 1],
        vec![1, 0, 1, 2, 2],
        vec![1, 0, 2, 1, 2],
        vec![1, 0, 2, 2, 1],
        vec![1, 1, 0, 2, 2],
        vec![1, 1, 2, 0, 2],
        vec![1, 1, 2, 2, 0],
        vec![1, 2, 0, 1, 2],
        vec![1, 2, 0, 2, 1],
        vec![1, 2, 1, 0, 2],
        vec![1, 2, 1, 2, 0],
        vec![1, 2, 2, 0, 1],
        vec![1, 2, 2, 1, 0],
        vec![2, 0, 1, 1, 2],
        vec![2, 0, 1, 2, 1],
        vec![2, 0, 2, 1, 1],
        vec![2, 1, 0, 1, 2],
        vec![2, 1, 0, 2, 1],
        vec![2, 1, 1, 0, 2],
        vec![2, 1, 1, 2, 0],
        vec![2, 1, 2, 0, 1],
        vec![2, 1, 2, 1, 0],
        vec![2, 2, 0, 1, 1],
        vec![2, 2, 1, 0, 1],
        vec![2, 2, 1, 1, 0],
    ];

    let fwd = Permutations::from(expected[0].clone()).forward(5);
    assert!(fwd.eq(expected.iter().cloned()));

    let fwd_1 = Permutations::from(expected[1].clone()).forward(5);
    assert!(fwd_1.eq(expected[1..].iter().cloned()));

    let fwd_28 = Permutations::from(expected[28].clone()).forward(5);
    assert!(fwd_28.eq(expected[28..].iter().cloned()));

    let fwd_29 = Permutations::from(expected[29].clone()).forward(5);
    assert!(fwd_29.eq(expected[29..].iter().cloned()));

    let bwd = Permutations::from(expected[0].clone()).backward(5);
    assert!(bwd.eq(expected[..=0].iter().rev().cloned()));

    let bwd_1 = Permutations::from(expected[1].clone()).backward(5);
    assert!(bwd_1.eq(expected[..=1].iter().rev().cloned()));

    let bwd_28 = Permutations::from(expected[28].clone()).backward(5);
    assert!(bwd_28.eq(expected[..=28].iter().rev().cloned()));

    let bwd_29 = Permutations::from(expected[29].clone()).backward(5);
    assert!(bwd_29.eq(expected[..=29].iter().rev().cloned()));
}

#[test]
fn empty() {
    let empty: Vec<()> = vec![];
    let fwd = Permutations::from(empty.clone()).forward(0);
    assert!(fwd.eq(Some(vec![])));
    let bwd = Permutations::from(empty.clone()).backward(0);
    assert!(bwd.eq(Some(vec![])));
}

#[test]
fn single() {
    let empty = vec![()];
    let fwd = Permutations::from(empty.clone()).forward(1);
    assert!(fwd.eq(Some(vec![()])));
    let bwd = Permutations::from(empty.clone()).backward(1);
    assert!(bwd.eq(Some(vec![()])));
}

#[cfg(test)]
fn is_sorted<T: Ord>(a: &[T]) -> bool { a.windows(2).all(|w| w[0] <= w[1]) }

#[test]
fn partial_fwd() {
    for n in 1..=8 {
        for k in 1..=n {
            let a: Vec<_> = (0..n).collect();
            let expected: Vec<_> = Permutations::from(a.clone())
                .forward(n)
                .filter(|p| is_sorted(&p[k..]))
                .map(|p| p[..k].to_vec())
                .collect();

            assert!(Permutations::from(a).forward(k).eq(expected));
        }
    }
}

#[test]
fn partial_bwd() {
    for n in 1..=8 {
        for k in 1..=n {
            let a: Vec<_> = (0..n).collect();
            let expected: Vec<_> = Permutations::from(a.clone())
                .forward(n)
                .filter(|p| is_sorted(&p[k..]))
                .map(|p| p[..k].to_vec())
                .collect();
            let expected: Vec<_> = expected.into_iter().rev().collect();

            let a_rev: Vec<_> = a.into_iter().rev().collect();
            assert!(Permutations::from(a_rev).backward(k).eq(expected));
        }
    }
}

#[test]
fn partial_dup() {
    let a = vec![0, 1, 1, 2, 2, 3, 3, 3];
    for n in 1..=8 {
        for k in 1..=n {
            let a: Vec<_> = a[..n].to_vec();
            let expected: Vec<_> = Permutations::from(a.clone())
                .forward(n)
                .filter(|p| is_sorted(&p[k..]))
                .map(|p| p[..k].to_vec())
                .collect();

            assert!(Permutations::from(a).forward(k).eq(expected));
        }
    }
}
