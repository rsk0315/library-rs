//! 法 $m$ での演算をする。

use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

use additive::{AddAssoc, AddComm, Zero};
use assoc_val::AssocVal;
use multiplicative::{MulAssoc, MulComm, MulRecip, One};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModInt<M: AssocVal<i64>>(i64, PhantomData<M>);

impl<M: AssocVal<i64>> Display for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<M: AssocVal<i64>> Debug for ModInt<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "[{} (mod {})]", self.0, M::get())
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl<M: AssocVal<i64>> ModInt<M> {
    pub fn get(&self) -> &i64 {
        &self.0
    }
}

macro_rules! impl_from {
    ( $t:ty ) => {
        impl<M: AssocVal<i64>> From<$t> for ModInt<M> {
            fn from(n: $t) -> Self {
                let n: i64 = n.into();
                let n = n % M::get();
                Self(n, PhantomData)
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
        Self(n, PhantomData)
    }
}

impl<M: AssocVal<i64>> Add for ModInt<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let n = match self.0 + other.0 {
            n if n >= M::get() => n - M::get(),
            n => n,
        };
        Self(n, PhantomData)
    }
}

impl<M: AssocVal<i64>> AddAssign for ModInt<M> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        match self.0 {
            n if n >= M::get() => self.0 -= M::get(),
            _ => {}
        };
    }
}

impl<M: AssocVal<i64>> Sub for ModInt<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let n = match self.0 - other.0 {
            n if n < 0 => n + M::get(),
            n => n,
        };
        Self(n, PhantomData)
    }
}

impl<M: AssocVal<i64>> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        match self.0 {
            n if n < 0 => self.0 += M::get(),
            _ => {}
        };
    }
}

impl<M: AssocVal<i64>> Mul for ModInt<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let n = (self.0 * other.0).rem_euclid(M::get());
        Self(n, PhantomData)
    }
}

impl<M: AssocVal<i64>> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, other: Self) {
        self.0 = (self.0 * other.0).rem_euclid(M::get());
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
        Self(0, PhantomData)
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
        let n = match self.0 {
            0 => 0,
            n => M::get() - n,
        };
        Self(n, PhantomData)
    }
}

impl<M: AssocVal<i64>> MulRecip for ModInt<M> {
    type Output = Self;
    fn mul_recip(self) -> Self {
        let mut x = 0_i64;
        let mut y = 1_i64;
        let mut a = self.0;
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
        assert_eq!(b, 1, "{} has no reciprocal modulo {}", self.0, M::get());
        Self::from(x)
    }
}

impl<M: AssocVal<i64>> AddAssoc for ModInt<M> {}
impl<M: AssocVal<i64>> AddComm for ModInt<M> {}
impl<M: AssocVal<i64>> MulAssoc for ModInt<M> {}
impl<M: AssocVal<i64>> MulComm for ModInt<M> {}

#[macro_export]
macro_rules! impl_mod_int {
    ( $i:ident => $m:expr ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        struct $i {}
        impl AssocVal<i64> for $i {
            fn get() -> i64 { $m }
        }
    };
    ( $( $i:ident => $m:expr, )* ) => { $( impl_mod_int!($i => $m); )* };
    ( $( $i:ident => $m:expr ),* ) => { $( impl_mod_int!($i => $m); )* };
}

#[test]
fn test() {
    impl_mod_int!(Mod1e9p7 => 1_000_000_007);
    type Mi = ModInt<Mod1e9p7>;
    let x = Mi::from(1) / Mi::from(2);
    assert_eq!(format!("{:?}", x), "500000004");
    assert_eq!(format!("{:#?}", x), "[500000004 (mod 1000000007)]");
    assert_eq!(x.0, 500_000_004);
}
