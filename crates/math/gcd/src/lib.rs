//! 最大公約数。

/// 最大公約数。
///
/// # Complexity
/// $O(\\log(\\min\\{m, n\\}))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::Gcd;
///
/// assert_eq!(12_u32.gcd(18), 6);
/// assert_eq!(13_i32.gcd(-3), 1);
/// assert_eq!(60_u32.gcd(90).gcd(150), 30);
///
/// assert_eq!(0_u32.gcd(0), 0);
/// assert_eq!(0_u32.gcd(1), 1);
/// ```
pub trait Gcd {
    fn gcd(self, other: Self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl Gcd for $t {
            fn gcd(mut self, mut other: Self) -> Self {
                while other > 0 {
                    let tmp = self % other;
                    self = std::mem::replace(&mut other, tmp);
                }
                self
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

macro_rules! impl_int {
    ($t:ty) => {
        impl Gcd for $t {
            fn gcd(mut self, mut other: Self) -> Self {
                while other != 0 {
                    let tmp = self % other;
                    self = std::mem::replace(&mut other, tmp);
                }
                self.abs()
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_int!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
impl_int!(i8 i16 i32 i64 i128 isize);
