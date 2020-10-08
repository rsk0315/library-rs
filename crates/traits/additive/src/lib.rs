//! 加法に関するトレイトたちです。
//!
//! これを実装したクラスは `OpAdd` によって和を求められます。
//! 区間和を求めるデータ構造などに使います。

use std::ops::Add;

/// 加法の単位元 $0$ を定義する。
pub trait Zero: Add<Output = Self> + Sized {
    /// 加法の単位元 $0$ を返す。
    fn zero() -> Self;
}
/// 加法が結合法則を満たすことを示す。
///
/// $$ x, y, z \\in S \\implies (x + y) + z = x + (y + z). $$
pub trait AddAssoc: Add<Output = Self> + Sized {}
/// 加法が交換法則を満たすことを示す。
///
/// $$ x, y \\in S \\implies x + y = y + x. $$
pub trait AddComm: Add<Output = Self> + Sized {}

macro_rules! impl_trait {
    (
        $( impl ($T:ty) for { $( $U:ty ),* } $S:tt )*
    ) => {
        $( $( impl $T for $U $S )* )*
    };
}

impl_trait! {
    impl (Zero) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {
        fn zero() -> Self { 0 }
    }
    impl (AddAssoc) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
    impl (AddComm) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
}
