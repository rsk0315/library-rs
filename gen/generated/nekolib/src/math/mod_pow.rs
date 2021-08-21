//! 冪乗。

/// 冪乗。
///
/// $a^b \\bmod n$ を返す。
///
/// # Complexity
/// $O(\\log(b))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::mod_pow;
///
/// assert_eq!(mod_pow(3, 14, 10), 9);
/// assert_eq!(mod_pow(2, 11, 1024), 0);
/// assert_eq!(mod_pow(0, 0, 1), 0);
/// ```
pub trait ModPow {
    fn mod_pow(self, b: Self, n: Self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl ModPow for $t {
            fn mod_pow(self, mut b: Self, n: Self) -> Self {
                if n == 1 {
                    return 0; // in case 0^0
                }
                let mut res = 1;
                let mut a = self;
                while b > 0 {
                    if b & 1 == 1 {
                        res = res * a % n;
                    }
                    a = a * a % n;
                    b >>= 1;
                }
                res
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

#[test]
fn test() {
    for n in 1_u64..=30 {
        for a in 0..30 {
            let mut expected = 1 % n;
            for b in 0..30 {
                assert_eq!(a.mod_pow(b, n), expected);
                expected = expected * a % n;
            }
        }
    }
}
