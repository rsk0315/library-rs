//! 素因数分解。

/// 素因数分解。
///
/// $n = \\prod\_{p\_i:\\text{ prime}} p\_i^{e\_i}$ に対して、各
/// $(p\_i, e\_i)$ を $p\_i$ の昇順に返す。
///
/// # Complexity
/// $O(\\sqrt{n})$ time, $O(1)$ space.
///
/// # Examples
/// ```
/// use nekolib::math::Factors;
///
/// let n = 735134400_u64;
/// let fac: Vec<_> = n.factors().collect();
/// assert_eq!(fac, [(2, 6), (3, 3), (5, 2), (7, 1), (11, 1), (13, 1), (17, 1)]);
/// assert_eq!(fac.iter().map(|&(p, e)| p.pow(e)).product::<u64>(), n);
///
/// assert_eq!(1_u64.factors().next(), None);
/// ```
pub trait Factors {
    type Output;
    fn factors(self) -> Self::Output;
}

pub struct FactorsStruct<I> {
    i: I,
    n: I,
}

macro_rules! impl_factors_unit {
    ( $($ty:ty)* ) => { $(
        impl Factors for $ty {
            type Output = FactorsStruct<$ty>;
            fn factors(self) -> Self::Output {
                Self::Output { i: 2, n: self }
            }
        }
        impl Iterator for FactorsStruct<$ty> {
            type Item = ($ty, u32);
            fn next(&mut self) -> Option<($ty, u32)> {
                if self.n <= 1 || self.i == 0 {
                    return None;
                }
                while self.i * self.i <= self.n {
                    while self.n % self.i == 0 {
                        let mut e = 1;
                        self.n /= self.i;
                        while self.n % self.i == 0 {
                            self.n /= self.i;
                            e += 1;
                        }
                        return Some((self.i, e));
                    }
                    self.i += 1;
                }
                if self.i > 0 {
                    self.i = 0;
                    return Some((self.n, 1));
                }
                None
            }
        }
    )* };
}

impl_factors_unit! { u8 u16 u32 u64 u128 usize }

#[test]
fn test_small() {
    let suite: &[(u64, &[(u64, u32)])] = &[
        (0, &[]),
        (1, &[]),
        (2, &[(2, 1)]),
        (3, &[(3, 1)]),
        (4, &[(2, 2)]),
        (5, &[(5, 1)]),
        (10, &[(2, 1), (5, 1)]),
        (100, &[(2, 2), (5, 2)]),
        (200, &[(2, 3), (5, 2)]),
    ];
    for (n, expected) in suite {
        let actual: Vec<_> = n.factors().collect();
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
        let actual: Vec<_> = i.factors().collect();
        let expected = {
            let mut res = vec![];
            let mut j = i;
            while j > 1 {
                if res.last().map(|&(x, _): &(usize, u32)| x) == Some(lp[j]) {
                    res.last_mut().unwrap().1 += 1;
                } else {
                    res.push((lp[j], 1));
                }
                j /= lp[j];
            }
            res
        };
        assert_eq!(actual, expected);
    }
}
