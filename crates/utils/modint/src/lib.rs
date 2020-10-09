//! 法 $m$ での演算をする。

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::*;

use additive::*;
use modulo::Mod;
use multiplicative::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModInt<M: Mod> {
    n: i64,
    _m: PhantomData<M>,
}

impl<M: Mod> ModInt<M> {
    pub fn new(n: i64) -> Self {
        let n = n % M::get();
        Self { n, _m: PhantomData }
    }
}

impl<M: Mod> Add for ModInt<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let n = match self.n + other.n {
            n if n >= M::get() => n - M::get(),
            n => n,
        };
        Self { n, _m: PhantomData }
    }
}

impl<M: Mod> AddAssign for ModInt<M> {
    fn add_assign(&mut self, other: Self) {
        self.n += other.n;
        match self.n {
            n if n >= M::get() => self.n -= M::get(),
            _ => {}
        };
    }
}

impl<M: Mod> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let n = match self.n - other.n {
            n if n < 0 => n + M::get(),
            n => n,
        };
        Self { n, _m: PhantomData }
    }
}

impl<M: Mod> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, other: Self) {
        self.n -= other.n;
        match self.n {
            n if n < 0 => self.n += M::get(),
            _ => {}
        };
    }
}

impl<M: Mod> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let n = (self.n * other.n).rem_euclid(M::get());
        Self { n, _m: PhantomData }
    }
}

impl<M: Mod> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, other: Self) {
        self.n = (self.n * other.n).rem_euclid(M::get());
    }
}

impl<M: Mod> Div for ModInt<M> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self.mul(other.mul_recip())
    }
}

impl<M: Mod> DivAssign for ModInt<M> {
    fn div_assign(&mut self, other: Self) {
        self.mul_assign(other.mul_recip())
    }
}

impl<M: Mod> Neg for ModInt<M> {
    type Output = Self;
    fn neg(self) -> Self {
        let n = match self.n {
            0 => 0,
            n => M::get() - n,
        };
        Self { n, _m: PhantomData }
    }
}

impl<M: Mod> MulRecip for ModInt<M> {
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
            let tmp = x - q * u;
            x = u;
            u = tmp;
            let tmp = y - q * v;
            y = v;
            v = tmp;
            let tmp = b - q * a;
            b = a;
            a = tmp;
        }
        assert_eq!(b, 1, "{} has no reciprocal modulo {}", self.n, M::get());
        Self::new(x)
    }
}

impl<M: Mod> AddAssoc for ModInt<M> {}
impl<M: Mod> AddComm for ModInt<M> {}
impl<M: Mod> MulAssoc for ModInt<M> {}
impl<M: Mod> MulComm for ModInt<M> {}
