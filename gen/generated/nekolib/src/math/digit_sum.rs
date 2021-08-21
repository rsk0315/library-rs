//! 桁和。

/// 桁和。
///
/// # Examples
/// ```
/// use nekolib::math::DigitSum;
///
/// assert_eq!(2_u32.pow(15).digit_sum(10), 3 + 2 + 7 + 6 + 8);
/// assert_eq!(123_u32.digit_pow_sum(10, 2), 1*1 + 2*2 + 3*3);
/// assert_eq!(0xACE_u32.digit_sum(16), 0xA + 0xC + 0xE);
///
/// assert_eq!(12345_u32.digit_pow_sum(10, 0), 5);  // the number of digits
/// assert_eq!(0_u32.digit_pow_sum(10, 0), 1);
/// ```
pub trait DigitSum: Sized {
    fn digit_pow_sum(self, base: Self, exp: u32) -> Self;
    fn digit_sum(self, base: Self) -> Self { self.digit_pow_sum(base, 1) }
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl DigitSum for $t {
            fn digit_pow_sum(mut self, base: Self, exp: u32) -> Self {
                if self == 0 {
                    return if exp == 0 { 1 } else { 0 };
                }
                let mut res = 0;
                while self > 0 {
                    res += (self % base).pow(exp);
                    self /= base;
                }
                res
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
