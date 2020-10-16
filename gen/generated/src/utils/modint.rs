//! 法 $m$ での演算をする。

use super::super::traits::additive;
use super::super::traits::assoc_val;
use super::super::traits::multiplicative;

use std::convert::TryInto;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

use additive::{AddAssoc, AddComm, Zero};
use assoc_val::AssocVal;
use multiplicative::{MulAssoc, MulComm, MulRecip, One};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModInt<M: AssocVal<i64>> {
    n: i64,
    _m: PhantomData<M>,
}

macro_rules! impl_from {
    ( $t:ty ) => {
        impl<M: AssocVal<i64>> From<$t> for ModInt<M> {
            fn from(n: $t) -> Self {
                let n: i64 = n.into();
                let n = n % M::get();
                Self { n, _m: PhantomData }
            }
        }
    };
    ( $( $t:ty ),* ) => { $( impl_from!($t); )* };
}

impl_from!(i8, i16, i32, i64, u8, u16, u32);

impl<M: AssocVal<i64>> From<u64> for ModInt<M> {
    fn from(n: u64) -> Self {
        let m: u64 = M::get().try_into().unwrap();
        let n: i64 = (n % m).try_into().unwrap();
        Self { n, _m: PhantomData }
    }
}

impl<M: AssocVal<i64>> Add for ModInt<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let n = match self.n + other.n {
            n if n >= M::get() => n - M::get(),
            n => n,
        };
        Self { n, _m: PhantomData }
    }
}

impl<M: AssocVal<i64>> AddAssign for ModInt<M> {
    fn add_assign(&mut self, other: Self) {
        self.n += other.n;
        match self.n {
            n if n >= M::get() => self.n -= M::get(),
            _ => {}
        };
    }
}

impl<M: AssocVal<i64>> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let n = match self.n - other.n {
            n if n < 0 => n + M::get(),
            n => n,
        };
        Self { n, _m: PhantomData }
    }
}

impl<M: AssocVal<i64>> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, other: Self) {
        self.n -= other.n;
        match self.n {
            n if n < 0 => self.n += M::get(),
            _ => {}
        };
    }
}

impl<M: AssocVal<i64>> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let n = (self.n * other.n).rem_euclid(M::get());
        Self { n, _m: PhantomData }
    }
}

impl<M: AssocVal<i64>> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, other: Self) {
        self.n = (self.n * other.n).rem_euclid(M::get());
    }
}

impl<M: AssocVal<i64>> Div for ModInt<M> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self.mul(other.mul_recip())
    }
}

impl<M: AssocVal<i64>> DivAssign for ModInt<M> {
    fn div_assign(&mut self, other: Self) {
        self.mul_assign(other.mul_recip())
    }
}

impl<M: AssocVal<i64>> Zero for ModInt<M> {
    fn zero() -> Self {
        Self {
            n: 0,
            _m: PhantomData,
        }
    }
}

impl<M: AssocVal<i64>> One for ModInt<M> {
    fn one() -> Self {
        Self::from(1_i64) // in case mod = 1
    }
}

impl<M: AssocVal<i64>> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        let n = match self.n {
            0 => 0,
            n => M::get() - n,
        };
        Self { n, _m: PhantomData }
    }
}

impl<M: AssocVal<i64>> MulRecip for ModInt<M> {
    type Output = Self;
    fn mul_recip(self) -> Self {
        let mut x = 0_i64;
        let mut y = 1_i64;
        let mut a = self.n;
        let mut b = M::get();
        let mut u = y;
        let mut v = x;
        while a != 0 {
            let q = b / a;
            {
                let tmp = x - q * u;
                x = u;
                u = tmp;
            }
            {
                let tmp = y - q * v;
                y = v;
                v = tmp;
            }
            {
                let tmp = b - q * a;
                b = a;
                a = tmp;
            }
        }
        assert_eq!(b, 1, "{} has no reciprocal modulo {}", self.n, M::get());
        Self::from(x)
    }
}

impl<M: AssocVal<i64>> AddAssoc for ModInt<M> {}
impl<M: AssocVal<i64>> AddComm for ModInt<M> {}
impl<M: AssocVal<i64>> MulAssoc for ModInt<M> {}
impl<M: AssocVal<i64>> MulComm for ModInt<M> {}
