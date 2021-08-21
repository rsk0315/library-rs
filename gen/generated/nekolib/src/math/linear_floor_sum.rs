//! $ \\sum\_{i=0}\^{n-1} \\left\\lfloor\\frac{ai+b}{m}\\right\\rfloor. $

/// $ \\sum\_{i=0}\^{n-1} \\left\\lfloor\\frac{ai+b}{m}\\right\\rfloor. $
///
/// # Requirements
/// - $n \\ge 0$
/// - $m \\gt 0$
///
/// # Idea
/// あとで書く。
///
/// ## See also
/// - <https://rsk0315.hatenablog.com/entry/2020/12/13/231307>
/// - <https://atcoder.jp/contests/practice2/editorial/579>
///
/// # Examples
/// ```
/// use nekolib::math::LinearFloorSum;
///
/// assert_eq!(4_u128.linear_floor_sum(10, 6, 3), 3);
/// assert_eq!(6_u128.linear_floor_sum(5, 4, 3), 13);
/// assert_eq!(1_u128.linear_floor_sum(1, 0, 0), 0);
/// assert_eq!(31415_u128.linear_floor_sum(92653, 58979, 32384), 314095480);
/// assert_eq!(
///     1000000000_u128.linear_floor_sum(1000000000, 999999999, 999999999),
///     499999999500000000
/// );
/// assert_eq!(14_i128.linear_floor_sum(23, -7, -39), -58);
/// ```
pub trait LinearFloorSum {
    fn linear_floor_sum(self, m: Self, a: Self, b: Self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl LinearFloorSum for $t {
            fn linear_floor_sum(
                self,
                mut m: Self,
                mut a: Self,
                mut b: Self
            ) -> Self {
                let mut n = self;
                let mut res = 0;
                loop {
                    if a >= m {
                        res += n * (n - 1) / 2 * (a / m);
                        a %= m;
                    }
                    if b >= m {
                        res += n * (b / m);
                        b %= m;
                    }
                    let y = a * n + b;
                    if y < m {
                        break;
                    }
                    n = y / m;
                    b = y % m;
                    std::mem::swap(&mut m, &mut a);
                }
                res
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

macro_rules! impl_int {
    ($t:ty) => {
        impl LinearFloorSum for $t {
            fn linear_floor_sum(self, m: Self, a: Self, b: Self) -> Self {
                let n = self;
                let mut res = 0;
                assert!(m > 0);
                assert!(n >= 0);
                let a = if a < 0 {
                    let a2 = a.rem_euclid(m);
                    res -= n * (n - 1) / 2 * ((a2 - a) / m);
                    a2 as u128
                } else {
                    a as u128
                };
                let b = if b < 0 {
                    let b2 = b.rem_euclid(m);
                    res -= n * ((b2 - b) / m);
                    b2 as u128
                } else {
                    b as u128
                };
                res + (self as u128).linear_floor_sum(m as u128, a, b) as $t
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_int!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
impl_int!(i8 i16 i32 i64 i128 isize);
