use binop::*;
use multiplicative::*;

/// 乗法。[`std::ops::Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) により定義される。
/// 単位元は [`One`]、逆元は [`MulRecip`] で定義する。
/// 結合法則を満たすときは [`MulAssoc`]、交換法則を満たすときは [`MulComm`] を実装することで示す。
pub struct OpMul<T> {
    _t: std::marker::PhantomData<T>,
}

use std::ops::Mul;

impl<T> Magma for OpMul<T>
where
    T: Mul<Output = T> + Eq + Sized,
{
    type Set = T;
    fn op(x: Self::Set, y: Self::Set) -> Self::Set {
        x * y
    }
}
impl<T> Identity for OpMul<T>
where
    T: Mul<Output = T> + Eq + Sized + One,
{
    fn id() -> Self::Set {
        Self::Set::one()
    }
}
impl<T> PartialRecip for OpMul<T>
where
    T: Mul<Output = T> + Eq + Sized + MulRecip<Output = T>,
{
    fn partial_recip(x: Self::Set) -> Option<Self::Set> {
        Some(x.mul_recip())
    }
}
impl<T> Recip for OpMul<T> where
    T: Mul<Output = T> + Eq + Sized + MulRecip<Output = T>
{
}
impl<T> Associative for OpMul<T> where T: Mul<Output = T> + Eq + Sized + MulAssoc
{}
impl<T> Commutative for OpMul<T> where T: Mul<Output = T> + Eq + Sized + MulComm {}

use op_add::OpAdd;

macro_rules! impl_distributive {
    ( $T:ty ) => {
        impl Distributive<OpAdd<$T>> for OpMul<$T> {}
    };
    ( $( $T:ty, )* ) => { $( impl_distributive!($T); )* };
}

impl_distributive! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}
