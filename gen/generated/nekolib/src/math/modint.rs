use super::gcd_recip;
use super::mod_pow;
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};
use std::sync::atomic::{self, AtomicU32, AtomicU64};

use gcd_recip::GcdRecip;
use mod_pow::ModPow;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct StaticModInt<M> {
    val: u32,
    _phd: PhantomData<fn() -> M>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DynamicModInt<I> {
    val: u32,
    _phd: PhantomData<fn() -> I>,
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

macro_rules! impl_static_binop {
    ( $( ($trait:ident, $op_assign:ident, $op:ident), )* ) => { $(
        impl<M: Modulus> $trait<StaticModInt<M>> for StaticModInt<M> {
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
        impl<'a, M: Modulus> $trait<StaticModInt<M>> for &'a StaticModInt<M> {
            type Output = StaticModInt<M>;
            fn $op(self, rhs: StaticModInt<M>) -> StaticModInt<M> {
                let mut tmp = self.to_owned();
                tmp.$op_assign(rhs);
                tmp
            }
        }
        impl<'a, M: Modulus> $trait<&'a StaticModInt<M>> for &'a StaticModInt<M> {
            type Output = StaticModInt<M>;
            fn $op(self, rhs: &'a StaticModInt<M>) -> StaticModInt<M> {
                let mut tmp = self.to_owned();
                tmp.$op_assign(*rhs);
                tmp
            }
        }
    )* }
}

impl_static_binop! {
    (Add, add_assign, add),
    (Sub, sub_assign, sub),
    (Mul, mul_assign, mul),
    (Div, div_assign, div),
}

impl<M: Modulus> AddAssign for StaticModInt<M> {
    fn add_assign(&mut self, rhs: StaticModInt<M>) {
        self.val += rhs.val;
        if self.val >= Self::modulus() {
            self.val -= Self::modulus();
        }
    }
}

impl<'a, M: Modulus> AddAssign<&'a StaticModInt<M>> for StaticModInt<M> {
    fn add_assign(&mut self, rhs: &'a StaticModInt<M>) {
        self.val += rhs.val;
        if self.val >= StaticModInt::<M>::modulus() {
            self.val -= StaticModInt::<M>::modulus();
        }
    }
}

impl<M: Modulus> SubAssign for StaticModInt<M> {
    fn sub_assign(&mut self, rhs: StaticModInt<M>) {
        if self.val < rhs.val {
            self.val += StaticModInt::<M>::modulus();
        }
        self.val -= rhs.val;
    }
}

impl<'a, M: Modulus> SubAssign<&'a StaticModInt<M>> for StaticModInt<M> {
    fn sub_assign(&mut self, rhs: &'a StaticModInt<M>) {
        if self.val < rhs.val {
            self.val += StaticModInt::<M>::modulus();
        }
        self.val -= rhs.val;
    }
}

impl<M: Modulus> MulAssign for StaticModInt<M> {
    fn mul_assign(&mut self, rhs: StaticModInt<M>) {
        let tmp = (self.val as u64 * rhs.val as u64)
            % StaticModInt::<M>::modulus() as u64;
        self.val = tmp as u32;
    }
}

impl<'a, M: Modulus> MulAssign<&'a StaticModInt<M>> for StaticModInt<M> {
    fn mul_assign(&mut self, rhs: &'a StaticModInt<M>) {
        let tmp = (self.val as u64 * rhs.val as u64)
            % StaticModInt::<M>::modulus() as u64;
        self.val = tmp as u32;
    }
}

impl<M: Modulus> DivAssign for StaticModInt<M> {
    fn div_assign(&mut self, rhs: StaticModInt<M>) {
        self.mul_assign(rhs.recip())
    }
}

impl<'a, M: Modulus> DivAssign<&'a StaticModInt<M>> for StaticModInt<M> {
    fn div_assign(&mut self, rhs: &'a StaticModInt<M>) {
        self.mul_assign(rhs.recip())
    }
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

pub trait DynamicModIntId: 'static + Copy + Eq {
    fn barrett() -> &'static Barrett;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum DefaultId {}

pub struct Barrett {
    m: AtomicU32,
    im: AtomicU64,
}

impl DynamicModIntId for DefaultId {
    fn barrett() -> &'static Barrett {
        static BARRETT: Barrett = Barrett::default();
        &BARRETT
    }
}

impl Barrett {
    pub const fn new(m: u32) -> Self {
        Self {
            m: AtomicU32::new(m),
            im: AtomicU64::new(Self::im(m)),
        }
    }

    pub const fn default() -> Self { Self::new(1) }

    const fn im(m: u32) -> u64 {
        (0_u64.wrapping_sub(1) / m as u64).wrapping_add(1)
    }

    fn set(&self, m: u32) {
        self.m.store(m, atomic::Ordering::SeqCst);
        self.im.store(Self::im(m), atomic::Ordering::SeqCst);
    }

    fn modulus(&self) -> u32 { self.m.load(atomic::Ordering::SeqCst) }

    fn mul(&self, a: u32, b: u32) -> u32 {
        let m = self.m.load(atomic::Ordering::SeqCst);
        let im = self.m.load(atomic::Ordering::SeqCst);

        let z = a as u64 * b as u64;
        let x = ((z as u128 * im as u128) >> 64) as u64;
        let v = z.wrapping_sub(x.wrapping_mul(m as u64)) as u32;
        if m <= v { v.wrapping_add(m) } else { v }
    }
}

impl Default for Barrett {
    fn default() -> Self { Self::default() }
}

impl<I: DynamicModIntId> DynamicModInt<I> {
    pub fn modulus() -> u32 { I::barrett().modulus() }

    pub fn set_modulus(m: u32) {
        // (m - 1) + (m - 1) < 2 ** 32
        // m <= 2 ** 31
        if !(1..=1 << 31).contains(&m) {
            panic!("the modulus must be in range (1, 2**31)");
        }
        I::barrett().set(m);
    }
}

impl<I: DynamicModIntId> ModIntBase for DynamicModInt<I> {
    fn modulus() -> u32 { Self::modulus() }
    fn get(self) -> u32 { self.val }
    fn new(n: u32) -> Self {
        Self { val: (n % Self::modulus()), _phd: PhantomData }
    }
    unsafe fn new_unchecked(val: u32) -> Self {
        Self { val, _phd: PhantomData }
    }
}

macro_rules! impl_dynamic_binop {
    ( $( ($trait:ident, $op_assign:ident, $op:ident), )* ) => { $(
        impl<I: DynamicModIntId> $trait<DynamicModInt<I>> for DynamicModInt<I> {
            type Output = DynamicModInt<I>;
            fn $op(self, rhs: DynamicModInt<I>) -> DynamicModInt<I> {
                let mut tmp = self;
                tmp.$op_assign(rhs);
                tmp
            }
        }
        impl<'a, I: DynamicModIntId> $trait<&'a DynamicModInt<I>> for DynamicModInt<I> {
            type Output = DynamicModInt<I>;
            fn $op(self, rhs: &'a DynamicModInt<I>) -> DynamicModInt<I> {
                let mut tmp = self;
                tmp.$op_assign(*rhs);
                tmp
            }
        }
        impl<'a, I: DynamicModIntId> $trait<DynamicModInt<I>> for &'a DynamicModInt<I> {
            type Output = DynamicModInt<I>;
            fn $op(self, rhs: DynamicModInt<I>) -> DynamicModInt<I> {
                let mut tmp = self.to_owned();
                tmp.$op_assign(rhs);
                tmp
            }
        }
        impl<'a, I: DynamicModIntId> $trait<&'a DynamicModInt<I>> for &'a DynamicModInt<I> {
            type Output = DynamicModInt<I>;
            fn $op(self, rhs: &'a DynamicModInt<I>) -> DynamicModInt<I> {
                let mut tmp = self.to_owned();
                tmp.$op_assign(*rhs);
                tmp
            }
        }
    )* }
}

impl_dynamic_binop! {
    (Add, add_assign, add),
    (Sub, sub_assign, sub),
    (Mul, mul_assign, mul),
    (Div, div_assign, div),
}

impl<I: DynamicModIntId> AddAssign for DynamicModInt<I> {
    fn add_assign(&mut self, rhs: DynamicModInt<I>) {
        self.val += rhs.val;
        if self.val >= DynamicModInt::<I>::modulus() {
            self.val -= DynamicModInt::<I>::modulus();
        }
    }
}

impl<'a, I: DynamicModIntId> AddAssign<&'a DynamicModInt<I>>
    for DynamicModInt<I>
{
    fn add_assign(&mut self, rhs: &'a DynamicModInt<I>) {
        self.val += rhs.val;
        if self.val >= DynamicModInt::<I>::modulus() {
            self.val -= DynamicModInt::<I>::modulus();
        }
    }
}

impl<I: DynamicModIntId> SubAssign for DynamicModInt<I> {
    fn sub_assign(&mut self, rhs: DynamicModInt<I>) {
        if self.val < rhs.val {
            self.val += DynamicModInt::<I>::modulus();
        }
        self.val -= rhs.val;
    }
}

impl<'a, I: DynamicModIntId> SubAssign<&'a DynamicModInt<I>>
    for DynamicModInt<I>
{
    fn sub_assign(&mut self, rhs: &'a DynamicModInt<I>) {
        if self.val < rhs.val {
            self.val += DynamicModInt::<I>::modulus();
        }
        self.val -= rhs.val;
    }
}

impl<I: DynamicModIntId> MulAssign for DynamicModInt<I> {
    fn mul_assign(&mut self, rhs: DynamicModInt<I>) {
        self.val = I::barrett().mul(self.val, rhs.val);
    }
}

impl<'a, I: DynamicModIntId> MulAssign<&'a DynamicModInt<I>>
    for DynamicModInt<I>
{
    fn mul_assign(&mut self, &rhs: &'a DynamicModInt<I>) {
        self.val = I::barrett().mul(self.val, rhs.val);
    }
}

impl<I: DynamicModIntId> DivAssign for DynamicModInt<I> {
    fn div_assign(&mut self, rhs: DynamicModInt<I>) {
        self.mul_assign(rhs.recip())
    }
}

impl<'a, I: DynamicModIntId> DivAssign<&'a DynamicModInt<I>>
    for DynamicModInt<I>
{
    fn div_assign(&mut self, rhs: &'a DynamicModInt<I>) {
        self.mul_assign(rhs.recip())
    }
}

impl<I: DynamicModIntId> Display for DynamicModInt<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<I: DynamicModIntId> Debug for DynamicModInt<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.val, Self::modulus())
    }
}

impl<I: DynamicModIntId> Sum<Self> for DynamicModInt<I> {
    fn sum<J: Iterator<Item = Self>>(iter: J) -> Self {
        iter.fold(Self::new(0), |x, y| x + y)
    }
}

impl<'a, I: DynamicModIntId> Sum<&'a Self> for DynamicModInt<I> {
    fn sum<J: Iterator<Item = &'a Self>>(iter: J) -> Self {
        iter.fold(Self::new(0), |x, y| x + y)
    }
}

impl<I: DynamicModIntId> Product<Self> for DynamicModInt<I> {
    fn product<J: Iterator<Item = Self>>(iter: J) -> Self {
        iter.fold(Self::new(1), |x, y| x * y)
    }
}

impl<'a, I: DynamicModIntId> Product<&'a Self> for DynamicModInt<I> {
    fn product<J: Iterator<Item = &'a Self>>(iter: J) -> Self {
        iter.fold(Self::new(1), |x, y| x * y)
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

    type Md = DynamicModInt<DefaultId>;
    Md::set_modulus(10);
    assert_eq!(Md::new(5) + Md::new(7), Md::new(2));

    Md::set_modulus(4);
    assert_eq!(Md::new(5) + Md::new(7), Md::new(0));

    let sum10: Md = (1..=10).map(Md::new).sum();
    assert_eq!(sum10, Md::new(55));
    assert_eq!(sum10.val, 55 % 4);
}

// todo:
// - static/dynamic
//     - butterfly
//     - impl DRY (use macro?)
// - static
//     - use IS_PRIME?
// - dynamic
//     - mod pow with Barrett reduction
