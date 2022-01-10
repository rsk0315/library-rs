//! 加法に関する wrapper クラス。

use super::super::traits::additive;
use super::super::traits::binop;

use std::fmt::Debug;

use additive::{AddAssoc, AddComm, Zero};
use binop::{Associative, Commutative, Identity, Magma};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpAddCount<T> {
    OpAddCountV,
    _Marker(T),
}
pub use OpAddCount::OpAddCountV;

impl<T> Default for OpAddCount<T> {
    fn default() -> Self { OpAddCountV }
}

use std::ops::Add;

impl<T> Magma for OpAddCount<T>
where
    T: Add<Output = T> + Eq + Sized,
{
    type Set = (T, T);
    fn op(&self, (xv, xc): Self::Set, (yv, yc): Self::Set) -> Self::Set {
        (xv + yv, xc + yc)
    }
}
impl<T> Identity for OpAddCount<T>
where
    T: Add<Output = T> + Eq + Sized + Zero,
{
    fn id(&self) -> Self::Set { (T::zero(), T::zero()) }
}
impl<T> Associative for OpAddCount<T> where
    T: Add<Output = T> + Eq + Sized + AddAssoc
{
}
impl<T> Commutative for OpAddCount<T> where
    T: Add<Output = T> + Eq + Sized + AddComm
{
}
