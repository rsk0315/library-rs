//! 加法に関する wrapper クラス。

use std::fmt::Debug;

use additive::*;
use binop::*;

/// 和を返す演算を持つ。
///
/// [`std::ops::Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) により定義される。
/// 単位元は [`Zero`]、逆元は [`std::ops::Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html) で定義する。
/// 結合法則を満たすときは [`AddAssoc`]、交換法則を満たすときは [`AddComm`] を実装することで示す。
///
/// [`Zero`]: ../../traits/additive/trait.Zero.html
/// [`AddAssoc`]: ../../traits/additive/trait.AddAssoc.html
/// [`AddComm`]: ../../traits/additive/trait.AddComm.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpAdd<T> {
    _t: std::marker::PhantomData<T>,
}

use std::ops::{Add, Neg};

impl<T> Magma for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized,
{
    type Set = T;
    fn op(x: Self::Set, y: Self::Set) -> Self::Set {
        x + y
    }
}
impl<T> Identity for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized + Zero,
{
    fn id() -> Self::Set {
        T::zero()
    }
}
impl<T> PartialRecip for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized + Neg<Output = T>,
{
    fn partial_recip(x: Self::Set) -> Option<Self::Set> {
        Some(-x)
    }
}
impl<T> Recip for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized + Neg<Output = T>,
{
    fn recip(x: Self::Set) -> Self::Set {
        -x
    }
}
impl<T> Associative for OpAdd<T> where T: Add<Output = T> + Eq + Sized + AddAssoc
{}
impl<T> Commutative for OpAdd<T> where T: Add<Output = T> + Eq + Sized + AddComm {}
