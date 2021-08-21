//! 最小公倍数。

use super::gcd;

/// 最小公倍数。
///
/// # Complexity
/// $O(\\log(\\min\\{m, n\\}))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::Lcm;
///
/// assert_eq!(12_u32.lcm(18), 36);
/// assert_eq!(13_i32.lcm(-3), -39);
/// assert_eq!(60_u32.lcm(90).lcm(150), 900);
///
/// assert_eq!(0_u32.lcm(0), 0);
/// assert_eq!(0_u32.lcm(1), 0);
/// ```
pub trait Lcm {
    fn lcm(self, other: Self) -> Self;
}

use gcd::Gcd;

macro_rules! impl_int {
    ($t:ty) => {
        impl Lcm for $t {
            fn lcm(self, other: Self) -> Self {
                if self == 0 || other == 0 {
                    0
                } else {
                    self / self.gcd(other) * other
                }
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_int!($t);)* };
}

impl_int!(u8 u16 u32 u64 u128 usize);
impl_int!(i8 i16 i32 i64 i128 isize);
