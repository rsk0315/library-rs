//! 最大値に関する wrapper クラス。

use std::fmt::Debug;

use binop::{Associative, Commutative, Identity, Magma};
use min::Min;

/// 最大値を返す演算を持つ。
///
/// 単位元は [`Min`] で定義する。
///
/// [`Min`]: ../../traits/min/trait.Min.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpMax<T> {
    _t: std::marker::PhantomData<T>,
}

impl<T> Magma for OpMax<T>
where
    T: Ord + Eq + Sized,
{
    type Set = T;
    fn op(x: Self::Set, y: Self::Set) -> Self::Set {
        x.max(y)
    }
}
impl<T> Identity for OpMax<T>
where
    T: Ord + Eq + Sized + Min,
{
    fn id() -> Self::Set {
        <T as Min>::min()
    }
}

impl<T> Associative for OpMax<T> where T: Ord + Eq + Sized {}
impl<T> Commutative for OpMax<T> where T: Ord + Eq + Sized {}
