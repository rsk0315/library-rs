//! 加法に関する wrapper クラス。

use std::fmt::Debug;

use additive::{AddAssoc, AddComm, Zero};
use binop::{Associative, Commutative, Identity, Magma, PartialRecip, Recip};

/// 和を返す演算を持つ。
///
/// [`std::ops::Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) により定義される。
/// 単位元は [`Zero`]、逆元は [`std::ops::Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html) で定義する。
/// 結合法則を満たすときは [`AddAssoc`]、交換法則を満たすときは [`AddComm`] を実装することで示す。
///
/// [`Zero`]: ../../traits/additive/trait.Zero.html
/// [`AddAssoc`]: ../../traits/additive/trait.AddAssoc.html
/// [`AddComm`]: ../../traits/additive/trait.AddComm.html
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpAdd<T> {
    OpAddV,
    _Marker(T),
}
pub use OpAdd::OpAddV;

impl<T> Default for OpAdd<T> {
    fn default() -> Self { OpAddV }
}

use std::ops::{Add, Neg};

impl<T> Magma for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized,
{
    type Set = T;
    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set { x + y }
}
impl<T> Identity for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized + Zero,
{
    fn id(&self) -> Self::Set { T::zero() }
}
impl<T> PartialRecip for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized + Neg<Output = T>,
{
    fn partial_recip(&self, x: Self::Set) -> Option<Self::Set> { Some(-x) }
}
impl<T> Recip for OpAdd<T>
where
    T: Add<Output = T> + Eq + Sized + Neg<Output = T>,
{
    fn recip(&self, x: Self::Set) -> Self::Set { -x }
}
impl<T> Associative for OpAdd<T> where T: Add<Output = T> + Eq + Sized + AddAssoc
{}
impl<T> Commutative for OpAdd<T> where T: Add<Output = T> + Eq + Sized + AddComm {}
