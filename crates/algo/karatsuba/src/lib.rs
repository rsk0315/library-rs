//! Karatsuba 法。

use std::ops::{AddAssign, Mul, SubAssign};

/// Karatsuba 法。Карацуба 法？
///
/// $a = (a\_i)$ と $b = (b\_i)$ の積 $a * b$ を求める。
/// ただし、$a * b$ は以下のように定義される。
/// $$ (a * b)\_i = \\sum\_{j=0}^i a\_j \\cdot b\_{i-j}. $$
///
/// # Idea
/// `todo!()`
///
/// # Complexity
/// $O(n^{\\log\_2(3)}) \\subset O(n^{1.585})$ time.
///
/// # Examples
/// ```
/// use nekolib::algo::multiply;
///
/// let a = vec![0_i32, 1, 2, 3, 4];
/// let b = vec![0, 1, 2, 4, 8];
/// assert_eq!(multiply(&a, &b), [0, 0, 1, 4, 11, 26, 36, 40, 32]);
/// ```
pub fn multiply<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: AddAssign + SubAssign + Mul<Output = T> + Default + Clone,
{
    let n = a.len();
    let m = b.len();
    let nm = n.max(m);
    let mut a = a.to_vec();
    a.resize_with(nm, T::default);
    let mut b = b.to_vec();
    b.resize_with(nm, T::default);
    let mut ab = mul(&mut a, &mut b);
    ab.truncate(n + m - 1);
    ab
}

const NAIVE_THRESHOLD: usize = 32;

fn mul<T>(a: &mut [T], b: &mut [T]) -> Vec<T>
where
    T: AddAssign + SubAssign + Mul<Output = T> + Default + Clone,
{
    assert_eq!(a.len(), b.len());

    let n = a.len();
    if n <= NAIVE_THRESHOLD {
        let mut res = vec![T::default(); n + n - 1];
        for (i, ai) in a.iter().enumerate() {
            for (j, bj) in b.iter().enumerate() {
                res[i + j] += ai.clone() * bj.clone();
            }
        }
        return res;
    }

    let nl = n / 2;
    let nh = n - nl;

    let (al, ah) = a.split_at_mut(nl);
    let (bl, bh) = b.split_at_mut(nl);

    let t = mul(al, bl);
    let u = mul(ah, bh);

    let mut alh = ah.to_vec();
    let mut blh = bh.to_vec();
    for i in 0..nl {
        alh[i] += al[i].clone();
        blh[i] += bl[i].clone();
    }

    let mut res = vec![T::default(); n + n - 1];
    let mut v = mul(&mut alh, &mut blh);
    for (i, ti) in t.iter().enumerate() {
        v[i] -= ti.clone();
        res[i] += ti.clone();
    }
    for (i, ui) in u.iter().enumerate() {
        v[i] -= ui.clone();
        res[nl + nl + i] += ui.clone();
    }

    if nl != nh {
        v.pop();
    }

    for (i, vi) in v.into_iter().enumerate() {
        res[nl + i] += vi;
    }
    res
}

#[test]
fn test() {
    let mut it =
        std::iter::successors(Some(14025256_i64), |&i| Some(i * i % 20300713))
            .map(|i| i % 10);
    let n = 1024;
    let a: Vec<_> = (0..n).map(|_| it.next().unwrap()).collect();
    let b: Vec<_> = (0..n).map(|_| it.next().unwrap()).collect();

    let mut expected = vec![0; n + n - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            expected[i + j] += ai * bj;
        }
    }

    let actual = multiply(&a, &b);
    assert_eq!(actual, expected);
}
