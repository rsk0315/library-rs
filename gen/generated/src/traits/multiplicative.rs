//! 乗法に関するトレイトたちです。
//!
//! これを実装したクラスは `OpMul` によって積を求められます。
//! 区間和を求めるデータ構造などに使います。

use std::ops::Mul;

/// 乗法の単位元 $1$ を定義する。
pub trait One: Mul<Output = Self> + Sized {
    /// 乗法の単位元 $1$ を返す。
    fn one() -> Self;
}
/// 乗法の逆元を定義する。
pub trait MulRecip {
    /// 返り値の型。
    type Output;
    /// 乗法における $x$ の逆元 $x^{-1}$ を返す。
    fn mul_recip(self) -> Self::Output;
}
/// 乗法が結合法則を満たすことを示す。
///
/// $$ x, y, z \\in S \\implies (x \\times y) \\times z = x \\times (y \\times z). $$
pub trait MulAssoc: Mul<Output = Self> + Sized {}
/// 乗法が交換法則を満たすことを示す。
///
/// $$ x, y \\in S \\implies x \\times y = y \\times x. $$
pub trait MulComm: Mul<Output = Self> + Sized {}

macro_rules! impl_trait {
    (
        $( impl ($T:ty) for { $( $U:ty ),* } $S:tt )*
    ) => {
        $( $( impl $T for $U $S )* )*
    };
}

impl_trait! {
    impl (One) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {
        fn one() -> Self { 1 }
    }
    impl (MulAssoc) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
    impl (MulComm) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
}
