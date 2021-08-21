//! 素因数分解。

/// 素因数分解。
///
/// $n = \\prod\_{p\_i:\\text{ prime}} p\_i^{e\_i}$ に対して、各
/// $(p\_i, e\_i)$ を $p\_i$ の昇順に返す。
///
/// $n = \\prod\_{p\_i:\\text{ prime}} p\_i^{e\_i}$ に対して、各
/// $p\_i$ を $e\_i$ 個、$p\_i$ の昇順に返す。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
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
///
/// ```
/// use nekolib::math::Factors;
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
pub trait Factors: Sized {
    // impl Iterator<Item = (Self, u32)>
    fn factors(self) -> std::vec::IntoIter<(Self, u32)>;
    // impl Iterator<Item = Self>
    fn factors_dup(self) -> std::vec::IntoIter<Self>;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl Factors for $t {
            fn factors(self) -> std::vec::IntoIter<(Self, u32)> {
                let mut res = vec![];
                let mut n = self;
                for i in std::iter::once(2).chain((3..).step_by(2)) {
                    if i * i > n {
                        break;
                    }
                    if n % i != 0 {
                        continue;
                    }
                    let mut e = 0;
                    while n % i == 0 {
                        n /= i;
                        e += 1;
                    }
                    res.push((i, e));
                }
                if n > 1 {
                    res.push((n, 1));
                }
                res.into_iter()
            }
            fn factors_dup(self) -> std::vec::IntoIter<Self> {
                self.factors().flat_map(|(p, e)| {
                    std::iter::repeat(p).take(e as usize)
                }).collect::<Vec<_>>().into_iter()
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
