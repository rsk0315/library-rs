use binop::*;
use max::Max;

#[derive(Clone)]
pub struct OpMin<T> {
    _t: std::marker::PhantomData<T>,
}

impl<T> Magma for OpMin<T>
where
    T: Ord + Eq + Sized,
{
    type Set = T;
    fn op(x: Self::Set, y: Self::Set) -> Self::Set {
        x.min(y)
    }
}
impl<T> Identity for OpMin<T>
where
    T: Ord + Eq + Sized + Max,
{
    fn id() -> Self::Set {
        <T as Max>::max()
    }
}

impl<T> Associative for OpMin<T> where T: Ord + Eq + Sized {}
impl<T> Commutative for OpMin<T> where T: Ord + Eq + Sized {}
