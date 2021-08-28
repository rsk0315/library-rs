//! 平方根。

/// 平方根。
///
/// $\\lfloor\\sqrt{n}\\rfloor$ を求める。
///
/// # Complexity
/// $O(\\log(n))$ time.
///
/// Newton 法で $O(\\log(\\log(n)))$ time にするべき？
///
/// # Examples
/// ```
/// use nekolib::math::Sqrt;
///
/// assert_eq!(0_i32.sqrt(), 0);
/// assert_eq!(9_i32.sqrt(), 3);
/// assert_eq!(12_i32.sqrt(), 3);
/// assert_eq!(16_i32.sqrt(), 4);
///
/// assert_eq!(u128::MAX.sqrt(), (1 << 64) - 1);
/// ```
///
/// ```should_panic
/// use nekolib::math::Sqrt;
///
/// (-1_i32).sqrt();
/// ```
pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl Sqrt for $t {
            fn sqrt(self) -> Self {
                let pred = |i: $t| {
                    i.checked_pow(2).map(|i2| i2 <= self).unwrap_or(false)
                };
                let mut ok = 0;
                let mut bad = 1;
                while pred(bad) {
                    bad *= 2;
                }
                while bad - ok > 1 {
                    let mid = ok + (bad - ok) / 2;
                    *(if pred(mid) { &mut ok } else { &mut bad }) = mid;
                }
                ok
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

macro_rules! impl_int {
    ($t:ty) => {
        impl Sqrt for $t {
            fn sqrt(self) -> Self {
                assert!(self >= 0, "must be non-negative");
                let pred = |i: $t| {
                    i.checked_pow(2).map(|i2| i2 <= self).unwrap_or(false)
                };
                let mut ok = 0;
                let mut bad = 1;
                while pred(bad) {
                    bad *= 2;
                }
                while bad - ok > 1 {
                    let mid = ok + (bad - ok) / 2;
                    *(if pred(mid) { &mut ok } else { &mut bad }) = mid;
                }
                ok
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_int!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
impl_int!(i8 i16 i32 i64 i128 isize);
