//! 最小値に関する wrapper クラス。

use std::fmt::Debug;

use binop::{Associative, Commutative, Identity, Magma};
use max::Max;

/// 最小値を返す演算を持つ。
///
/// 単位元は [`Max`] で定義する。
///
/// [`Max`]: ../../traits/max/trait.Max.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpMin<T> {
    OpMinV,
    _Marker(T),
}
pub use OpMin::OpMinV;

impl<T> Default for OpMin<T> {
    fn default() -> Self { OpMinV }
}

impl<T> Magma for OpMin<T>
where
    T: Ord + Eq + Sized,
{
    type Set = T;
    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set { x.min(y) }
}
impl<T> Identity for OpMin<T>
where
    T: Ord + Eq + Sized + Max,
{
    fn id(&self) -> Self::Set { <T as Max>::max() }
}

impl<T> Associative for OpMin<T> where T: Ord + Eq + Sized {}
impl<T> Commutative for OpMin<T> where T: Ord + Eq + Sized {}
