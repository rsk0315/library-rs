use std::convert::From;
use std::iter::{IntoIterator, Iterator};
use std::ops::{Index, RangeBounds};

use binop::{Magma, Monoid};
use buf_range::bounds_within;
use fold::Fold;
use set_value::SetValue;

#[derive(Clone)]
pub struct VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    buf: Vec<<M as Magma>::Set>,
    len: usize,
}

impl<M> VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    pub fn new(len: usize) -> Self {
        Self {
            len,
            buf: vec![M::id(); len + len],
        }
    }
}

impl<M, B> Fold<B> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
    B: RangeBounds<usize>,
{
    type Output = M;
    fn fold(&self, b: B) -> <M as Magma>::Set {
        let mut resl = M::id();
        let mut resr = M::id();
        let b = bounds_within(b, self.len);
        let (mut il, mut ir) = (b.start + self.len, b.end + self.len);
        while il < ir {
            if il & 1 == 1 {
                resl = <M as Magma>::op(resl, self.buf[il].clone());
                il += 1;
            }
            if ir & 1 == 1 {
                ir -= 1;
                resr = <M as Magma>::op(self.buf[ir].clone(), resr);
            }
            il >>= 1;
            ir >>= 1;
        }
        <M as Magma>::op(resl, resr)
    }
}

impl<M> SetValue<usize> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    type Input = <M as Magma>::Set;
    fn set_value(&mut self, i: usize, x: Self::Input) {
        assert!(i <= self.len, "index should be in {:?}", 0..self.len);
        let mut i = i + self.len;
        self.buf[i] = x;
        while i > 1 {
            i >>= 1;
            self.buf[i] = <M as Magma>::op(
                self.buf[i << 1].clone(),
                self.buf[i << 1 | 1].clone(),
            );
        }
    }
}

impl<M> From<Vec<<M as Magma>::Set>> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    fn from(mut v: Vec<<M as Magma>::Set>) -> Self {
        let len = v.len();
        let mut buf = vec![M::id(); len];
        buf.append(&mut v);
        for i in (0..len).rev() {
            buf[i] =
                <M as Magma>::op(buf[i << 1].clone(), buf[i << 1 | 1].clone());
        }
        Self { buf, len }
    }
}

impl<M> Index<usize> for VecSegtree<M>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    type Output = <M as Magma>::Set;
    fn index(&self, i: usize) -> &Self::Output {
        &self.buf[i + self.len]
    }
}

impl<M> From<VecSegtree<M>> for Vec<<M as Magma>::Set>
where
    M: Monoid,
    <M as Magma>::Set: Clone,
{
    fn from(v: VecSegtree<M>) -> Self {
        v.buf.into_iter().skip(v.len).collect()
    }
}

#[cfg(test)]
mod test {
    // use fold::Fold;
    use op_add::OpAdd;
    // use set_value::SetValue;

    use crate::*;

    #[test]
    fn test() {
        let mut s = VecSegtree::<OpAdd<i32>>::new(5);
        s.set_value(1usize, 3);
        s.set_value(4usize, 5);
        s.set_value(2usize, 2);
        s.set_value(0usize, 4);
        assert_eq!(Vec::<_>::from(s.clone()), vec![4, 3, 2, 0, 5]);
        assert_eq!(s.fold(0..4), 9);
        assert_eq!(s.fold(1..=3), 5);
        assert_eq!(s.fold(3..), 5);
        assert_eq!(s.fold(..0), 0);
        assert_eq!(s.fold(..=2), 9);
        assert_eq!(s.fold(..), 14);
        assert_eq!(s[1], 3);

        let s = VecSegtree::<OpAdd<i32>>::from(vec![1, 4, 2, 5, 3]);
        assert_eq!(Vec::<_>::from(s.clone()), vec![1, 4, 2, 5, 3]);
        assert_eq!(s.fold(1..3), 6);
        assert_eq!(s.fold(0..=2), 7);
        assert_eq!(s.fold(5..), 0);
        assert_eq!(s.fold(..3), 7);
        assert_eq!(s.fold(..=4), 15);
        assert_eq!(s.fold(..), 15);
        assert_eq!(s[4], 3);
    }

    #[test]
    #[should_panic(expected = "index")]
    fn test_out_of_index() {
        let mut s = VecSegtree::<OpAdd<i32>>::from(vec![1, 4, 2, 5, 3]);
        s.set_value(5, 2);
    }
}
