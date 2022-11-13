use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

use gcd_recip::GcdRecip;
use mod_pow::ModPow;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct StaticModInt<M> {
    val: u32,
    _phd: PhantomData<fn() -> M>,
}

pub trait Modulus: 'static + Copy + Eq {
    const VALUE: u32;
    const IS_PRIME: bool = is_prime_32(Self::VALUE);
}

pub trait ModIntBase:
    Copy
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    fn modulus() -> u32;
    fn get(self) -> u32;
    fn new(n: u32) -> Self;
    unsafe fn new_unchecked(n: u32) -> Self;
    fn recip(self) -> Self { self.checked_recip().unwrap() }
    fn checked_recip(self) -> Option<Self> {
        let (g, r) = (self.get() as u64).gcd_recip(Self::modulus() as u64);
        if g == 1 {
            Some(unsafe { Self::new_unchecked(r as u32) })
        } else {
            None
        }
    }
    fn pow(self, iexp: u64) -> Self {
        unsafe {
            Self::new_unchecked(
                (self.get() as u64).mod_pow(iexp, Self::modulus() as u64)
                    as u32,
            )
        }
    }
}

impl<M: Modulus> StaticModInt<M> {
    fn modulus() -> u32 { M::VALUE }
}

impl<M: Modulus> ModIntBase for StaticModInt<M> {
    fn modulus() -> u32 { Self::modulus() }
    fn get(self) -> u32 { self.val }
    fn new(n: u32) -> Self {
        Self { val: (n % Self::modulus()), _phd: PhantomData }
    }
    unsafe fn new_unchecked(val: u32) -> Self {
        Self { val, _phd: PhantomData }
    }
}

macro_rules! impl_binop {
    ( $( ($trait:ident, $op_assign:ident, $op:ident), )* ) => { $(
        impl<M: Modulus> $trait for StaticModInt<M> {
            type Output = StaticModInt<M>;
            fn $op(self, rhs: StaticModInt<M>) -> StaticModInt<M> {
                let mut tmp = self;
                tmp.$op_assign(rhs);
                tmp
            }
        }
        impl<'a, M: Modulus> $trait<&'a StaticModInt<M>> for StaticModInt<M> {
            type Output = StaticModInt<M>;
            fn $op(self, rhs: &'a StaticModInt<M>) -> StaticModInt<M> {
                let mut tmp = self;
                tmp.$op_assign(*rhs);
                tmp
            }
        }
        impl<'a, M: Modulus> $trait<StaticModInt<M>> for &'a StaticModInt< M> {
            type Output = StaticModInt<M>;
            fn $op(self, rhs: StaticModInt<M>) -> StaticModInt<M> {
                let mut tmp = self.to_owned();
                tmp.$op_assign(rhs);
                tmp
            }
        }
        impl<'a, M: Modulus> $trait<&'a StaticModInt<M>> for &'a StaticModInt< M> {
            type Output = StaticModInt<M>;
            fn $op(self, rhs: &'a StaticModInt<M>) -> StaticModInt<M> {
                let mut tmp = self.to_owned();
                tmp.$op_assign(*rhs);
                tmp
            }
        }
    )* }
}

impl_binop! {
    (Add, add_assign, add),
    (Sub, sub_assign, sub),
    (Mul, mul_assign, mul),
    (Div, div_assign, div),
}

impl<M: Modulus> AddAssign for StaticModInt<M> {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= Self::modulus() {
            self.val -= Self::modulus();
        }
    }
}

impl<M: Modulus> SubAssign for StaticModInt<M> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.val < rhs.val {
            self.val += Self::modulus();
        }
        self.val -= rhs.val;
    }
}

impl<M: Modulus> MulAssign for StaticModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        let tmp = (self.val as u64 * rhs.val as u64) % Self::modulus() as u64;
        self.val = tmp as u32;
    }
}

impl<M: Modulus> DivAssign for StaticModInt<M> {
    fn div_assign(&mut self, rhs: Self) { self.mul_assign(rhs.recip()) }
}

impl<M: Modulus> Display for StaticModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<M: Modulus> Debug for StaticModInt<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.val, Self::modulus())
    }
}

impl<M: Modulus> Sum<Self> for StaticModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |x, y| x + y)
    }
}

impl<'a, M: Modulus> Sum<&'a Self> for StaticModInt<M> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |x, y| x + y)
    }
}

impl<M: Modulus> Product<Self> for StaticModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |x, y| x * y)
    }
}

impl<'a, M: Modulus> Product<&'a Self> for StaticModInt<M> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |x, y| x * y)
    }
}

const fn is_sprp_32(n: u32, a: u32) -> bool {
    let n = n as u64;
    let s = (n - 1).trailing_zeros();
    let d = n >> s;
    let mut cur = {
        let mut cur = 1;
        let mut pow = d;
        let mut a = a as u64;
        while pow > 0 {
            if pow & 1 != 0 {
                cur = cur * a % n;
            }
            a = a * a % n;
            pow >>= 1;
        }
        cur
    };
    if cur == 1 {
        return true;
    }
    let mut i = 0;
    while i < s {
        if cur == n - 1 {
            return true;
        }
        cur = cur * cur % n;
        i += 1;
    }
    false
}

const fn is_prime_32(n: u32) -> bool {
    if n == 2 || n == 3 || n == 5 || n == 7 {
        true
    } else if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        false
    } else if n < 121 {
        n > 1
    } else {
        is_sprp_32(n, 2) && is_sprp_32(n, 7) && is_sprp_32(n, 61)
    }
}

macro_rules! impl_modint {
    ( $( ($mod:ident, $val:literal, $modint:ident), )* ) => { $(
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub struct $mod;
        impl Modulus for $mod {
            const VALUE: u32 = $val;
        }
        pub type $modint = StaticModInt<$mod>;
    )* }
}

impl_modint! {
    (Mod998244353, 998244353, ModInt998244353),
    (Mod1000000007, 1000000007, ModInt1000000007),
}

#[test]
fn sanity_check() {
    assert!(Mod998244353::IS_PRIME);
    assert!(Mod1000000007::IS_PRIME);

    type Mi = ModInt998244353;
    assert_eq!(Mi::new(1) + Mi::new(998244352), Mi::new(0));
    assert_eq!((Mi::new(1) / Mi::new(2)).get(), (Mi::modulus() + 1) / 2);

    let sum10: Mi = (1..=10).map(Mi::new).sum();
    assert_eq!(sum10, Mi::new(55));

    let prod10: Mi = (1..=10).map(Mi::new).product();
    assert_eq!(prod10, Mi::new(3628800));
}

// todo:
// - static/dynamic
//     - butterfly
// - dynamic
//     - barrett
