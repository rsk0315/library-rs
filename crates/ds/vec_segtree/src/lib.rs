use std::convert::{From, FromIterator};
use std::ops::Index;

use binop::{Magma, Monoid};
use fold::Fold;
use set_value::SetValue;

pub struct VecSegtree<M: Monoid> {
    buf: Vec<<M as Magma>::Set>,
    len: usize,
}

impl<M: Monoid> Fold<RangeBounds<usize>> for VecSegtree<M> {
    //
}

impl<M: Monoid> SetValue<usize> for VecSegtree<M> {
    //
}

impl<M: Monoid> From<Vec<<M as Magma>::Set>> for VecSegtree<M> {
    //
}

impl<M: Monoid> FromIterator<<M as Magma>::Set> for VecSegtree<M> {
    //
}

impl<M: Monoid> Index<usize> for VecSegtree<M> {
    //
}

impl<M: Monoid> Iter for VecSegtree<M> {
    //
}

impl<M: Monoid> IntoIterator for VecSegtree<M> {
    //
}
