use std::fmt::Debug;

use additive::Zero;
use binop::{Associative, Commutative, Identity, Magma};
use gcd::Gcd;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpGcd<T> {
    OpGcdV,
    _Marker(T),
}
pub use OpGcd::OpGcdV;

impl<T> Default for OpGcd<T> {
    fn default() -> Self { OpGcdV }
}

impl<T> Magma for OpGcd<T>
where
    T: Gcd + Eq + Sized,
{
    type Set = T;
    fn op(&self, x: Self::Set, y: Self::Set) -> Self::Set { x.gcd(y) }
}
impl<T> Identity for OpGcd<T>
where
    T: Gcd + Eq + Zero + Sized,
{
    fn id(&self) -> Self::Set { T::zero() }
}

impl<T> Associative for OpGcd<T> where T: Gcd + Eq + Sized {}
impl<T> Commutative for OpGcd<T> where T: Gcd + Eq + Sized {}
