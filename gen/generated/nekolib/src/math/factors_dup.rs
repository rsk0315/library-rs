//! 素因数分解。

/// 素因数分解。
///
/// $n = \\prod\_{p\_i:\\text{ prime}} p\_i^{e\_i}$ に対して、各
/// $p\_i$ を $e\_i$ 個、$p\_i$ の昇順に返す。
///
/// # Complexity
/// $O(\\sqrt{n})$ time, $O(1)$ space.
///
/// ```
/// use nekolib::math::FactorsDup;
///
/// let n = 735134400_u64;
/// let fac: Vec<_> = n.factors_dup().collect();
/// assert_eq!(fac, [2, 2, 2, 2, 2, 2, 3, 3, 3, 5, 5, 7, 11, 13, 17]);
/// assert_eq!(fac.iter().product::<u64>(), n);
///
/// assert_eq!(
///     (2_u64.pow(5) * 3_u64.pow(5)).factors_dup().product::<u64>(),
///     6_u64.pow(5)
/// );
///
/// assert_eq!(1_u64.factors_dup().next(), None);
/// ```
pub trait FactorsDup {
    type Output;
    fn factors_dup(self) -> Self::Output;
}

pub struct FactorsDupStruct<I> {
    i: I,
    n: I,
}

macro_rules! impl_factors_dup_uint {
    ( $($ty:ty)* ) => { $(
        impl FactorsDup for $ty {
            type Output = FactorsDupStruct<$ty>;
            fn factors_dup(self) -> Self::Output {
                Self::Output { i: 2, n: self }
            }
        }
        impl Iterator for FactorsDupStruct<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.n <= 1 || self.i == 0 {
                    return None;
                }
                loop {
                    match self.i.checked_pow(2) {
                        Some(_) if self.n % self.i == 0 => {
                            self.n /= self.i;
                            return Some(self.i);
                        }
                        Some(ii) if ii <= self.n => {
                            self.i += 1;
                        }
                        _ => {
                            return Some(std::mem::take(&mut self.n));
                        }
                    }
                }
            }
        }
    )* };
}

impl_factors_dup_uint! { u8 u16 u32 u64 u128 usize }

#[test]
fn test_small() {
    let suite: &[(u64, &[u64])] = &[
        (0, &[]),
        (1, &[]),
        (2, &[2]),
        (3, &[3]),
        (4, &[2, 2]),
        (5, &[5]),
        (10, &[2, 5]),
        (100, &[2, 2, 5, 5]),
        (200, &[2, 2, 2, 5, 5]),
    ];
    for (n, expected) in suite {
        let actual: Vec<_> = n.factors_dup().collect();
        assert_eq!(&actual, expected);
    }
}

#[test]
fn test() {
    let n = 10000_usize;

    let lp = {
        let mut lp: Vec<_> = (0..=n).collect();
        for i in 2..=n {
            if lp[i] < i {
                continue;
            }
            for j in i..=n / i {
                if lp[i * j] == i * j {
                    lp[i * j] = i;
                }
            }
        }
        lp
    };

    for i in 0..=n {
        let actual: Vec<_> = i.factors_dup().collect();
        let expected = {
            let mut res = vec![];
            let mut j = i;
            while j > 1 {
                res.push(lp[j]);
                j /= lp[j];
            }
            res
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn overflow() {
    for i in (1_u32..=1000)
        .flat_map(|i| [i.wrapping_neg(), 2_u32.pow(16) * (2_u32.pow(16) - i)])
    {
        let actual: Vec<_> = i.factors_dup().collect();
        let expected: Vec<_> =
            (i as u64).factors_dup().map(|d| d as u32).collect();
        assert_eq!(actual, expected);
    }
}

#[test]
fn overflow_exhaustive() {
    for i in u8::MIN..=u8::MAX {
        let actual: Vec<_> = i.factors_dup().collect();
        let expected: Vec<_> =
            (i as u32).factors_dup().map(|d| d as u8).collect();
        assert_eq!(actual, expected);
    }
    for i in u16::MIN..=u16::MAX {
        let actual: Vec<_> = i.factors_dup().collect();
        let expected: Vec<_> =
            (i as u32).factors_dup().map(|d| d as u16).collect();
        assert_eq!(actual, expected);
    }
}
