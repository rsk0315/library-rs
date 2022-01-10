//! 加法に関する wrapper クラス。

use super::super::traits::additive;
use super::super::traits::binop;
use super::super::traits::multiplicative;

use std::fmt::Debug;

use additive::{AddAssoc, Zero};
use binop::{Associative, Identity, Magma};
use multiplicative::One;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpAffine<T> {
    OpAffineV,
    _Marker(T),
}
pub use OpAffine::OpAffineV;

impl<T> Default for OpAffine<T> {
    fn default() -> Self { OpAffineV }
}

use std::ops::{Add, Mul};

impl<T> Magma for OpAffine<T>
where
    T: Add<Output = T> + Mul<Output = T> + Eq + Clone + Sized,
{
    type Set = (T, T);
    fn op(&self, (x1, x0): Self::Set, (y1, y0): Self::Set) -> Self::Set {
        // c(ax+b) + d = acx + (bc+d)
        let z1 = x1 * y1.clone();
        let z0 = x0 * y1 + y0;
        (z1, z0)
    }
}
impl<T> Identity for OpAffine<T>
where
    T: Add<Output = T> + Mul<Output = T> + Eq + Clone + Sized + Zero + One,
{
    fn id(&self) -> Self::Set { (T::one(), T::zero()) }
}
impl<T> Associative for OpAffine<T> where
    T: Add<Output = T> + Mul<Output = T> + Eq + Clone + Sized + AddAssoc
{
}
