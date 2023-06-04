use super::gcd_recip;
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};
use std::sync::atomic::{self, AtomicU32, AtomicU64};

use gcd_recip::GcdRecip;

#[derive(Copy, Clone, Eq, PartialEq)]
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
    #[cfg(ignore)]
    const IS_PRIME: bool = is_prime_32(Self::VALUE);
}

pub trait ModIntBase:
    Copy
    + Eq
    + Hash
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    fn modulus() -> u32;
    fn get(self) -> u32;
    fn new(n: impl RemEuclidU32) -> Self {
        let n = n.rem_euclid_u32(Self::modulus());
        unsafe { Self::new_unchecked(n) }
    }
    unsafe fn new_unchecked(n: u32) -> Self;
    fn recip(self) -> Self { self.checked_recip().unwrap() }
    fn checked_recip(self) -> Option<Self> {
        let (g, r) = (self.get() as u64).gcd_recip(Self::modulus() as u64);
        let r = r as u32;
        if g == 1 { Some(unsafe { Self::new_unchecked(r) }) } else { None }
    }
    fn pow(self, mut iexp: u64) -> Self {
        let mut res = Self::new(1);
        let mut a = self;
        while iexp > 0 {
            if iexp & 1 != 0 {
                res *= a;
            }
            a *= a;
            iexp >>= 1;
        }
        res
    }
}

trait InternalImpls: ModIntBase {
    fn hash_impl(&self, state: &mut impl Hasher) { self.get().hash(state) }
    fn display_impl(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
    fn debug_impl(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.get(), Self::modulus())
    }
    fn neg_impl(self) -> Self {
        let v = if self.get() == 0 { 0 } else { Self::modulus() - self.get() };
        unsafe { Self::new_unchecked(v) }
    }
}

impl<M: Modulus> StaticModInt<M> {
    fn modulus() -> u32 { M::VALUE }
    fn zero() -> Self { unsafe { Self::new_unchecked(0) } }
    fn add_impl(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp += rhs;
        tmp
    }
    fn sub_impl(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp -= rhs;
        tmp
    }
    fn mul_impl(self, rhs: Self) -> Self {
        let v = ((self.val as u64 * rhs.val as u64) % Self::modulus() as u64)
            as u32;
        unsafe { Self::new_unchecked(v) }
    }
    fn div_impl(self, rhs: Self) -> Self { self.mul_impl(rhs.recip()) }
    fn add_assign_impl(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= Self::modulus() {
            self.val -= Self::modulus()
        }
    }
    fn sub_assign_impl(&mut self, rhs: Self) {
        if self.val < rhs.val {
            self.val += Self::modulus()
        }
        self.val -= rhs.val
    }
    fn mul_assign_impl(&mut self, rhs: Self) { *self = self.mul_impl(rhs) }
    fn div_assign_impl(&mut self, rhs: Self) { *self = self.div_impl(rhs) }
}

impl<M: Modulus> ModIntBase for StaticModInt<M> {
    fn modulus() -> u32 { Self::modulus() }
    fn get(self) -> u32 { self.val }
    unsafe fn new_unchecked(val: u32) -> Self {
        Self { val, _phd: PhantomData }
    }
}

impl<M: Modulus> InternalImpls for StaticModInt<M> {}

impl<I: RemEuclidU32, M: Modulus> From<I> for StaticModInt<M> {
    fn from(x: I) -> Self { Self::new(x) }
}

#[cfg(ignore)]
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

#[cfg(ignore)]
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
    fn zero() -> Self { unsafe { Self::new_unchecked(0) } }
    fn add_impl(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp += rhs;
        tmp
    }
    fn sub_impl(self, rhs: Self) -> Self {
        let mut tmp = self;
        tmp -= rhs;
        tmp
    }
    fn mul_impl(self, rhs: Self) -> Self {
        let v = I::barrett().mul(self.val, rhs.val);
        unsafe { Self::new_unchecked(v) }
    }
    fn div_impl(self, rhs: Self) -> Self { self.mul_impl(rhs.recip()) }
    fn add_assign_impl(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= Self::modulus() {
            self.val -= Self::modulus()
        }
    }
    fn sub_assign_impl(&mut self, rhs: Self) {
        if self.val < rhs.val {
            self.val += Self::modulus()
        }
        self.val -= rhs.val
    }
    fn mul_assign_impl(&mut self, rhs: Self) { *self = self.mul_impl(rhs) }
    fn div_assign_impl(&mut self, rhs: Self) { *self = self.div_impl(rhs) }

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
    unsafe fn new_unchecked(val: u32) -> Self {
        Self { val, _phd: PhantomData }
    }
}

impl<I: DynamicModIntId> InternalImpls for DynamicModInt<I> {}

impl<J: RemEuclidU32, I: DynamicModIntId> From<J> for DynamicModInt<I> {
    fn from(x: J) -> Self { Self::new(x) }
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

macro_rules! impl_bin_ops {
    () => {};
    (
        for<$($generic_param:ident : $bound:tt),*>
            <$lhs_ty:ty> @ <$rhs_ty:ty> -> $output:ty
        { self @ $($rhs_deref:tt)? } $($rest:tt)*
    ) => {
        impl <$($generic_param: $bound),*> Add<$rhs_ty> for $lhs_ty {
            type Output = $output;
            fn add(self, rhs: $rhs_ty) -> $output { self.add_impl($($rhs_deref)? rhs) }
        }
        impl <$($generic_param: $bound),*> Sub<$rhs_ty> for $lhs_ty {
            type Output = $output;
            fn sub(self, rhs: $rhs_ty) -> $output { self.sub_impl($($rhs_deref)? rhs) }
        }
        impl <$($generic_param: $bound),*> Mul<$rhs_ty> for $lhs_ty {
            type Output = $output;
            fn mul(self, rhs: $rhs_ty) -> $output { self.mul_impl($($rhs_deref)? rhs) }
        }
        impl <$($generic_param: $bound),*> Div<$rhs_ty> for $lhs_ty {
            type Output = $output;
            fn div(self, rhs: $rhs_ty) -> $output { self.div_impl($($rhs_deref)? rhs) }
        }
        impl_bin_ops!($($rest)*);
    };
}

macro_rules! impl_assign_ops {
    () => {};
    (
        for<$($generic_param:ident : $bound:tt),*>
            <$lhs_ty:ty> @= <$rhs_ty:ty>
        { self @= $($rhs_deref:tt)? } $($rest:tt)*
    ) => {
        impl <$($generic_param: $bound),*> AddAssign<$rhs_ty> for $lhs_ty {
            fn add_assign(&mut self, rhs: $rhs_ty) {
                self.add_assign_impl($($rhs_deref)? rhs);
            }
        }
        impl <$($generic_param: $bound),*> SubAssign<$rhs_ty> for $lhs_ty {
            fn sub_assign(&mut self, rhs: $rhs_ty) {
                self.sub_assign_impl($($rhs_deref)? rhs);
            }
        }
        impl <$($generic_param: $bound),*> MulAssign<$rhs_ty> for $lhs_ty {
            fn mul_assign(&mut self, rhs: $rhs_ty) {
                self.mul_assign_impl($($rhs_deref)? rhs);
            }
        }
        impl <$($generic_param: $bound),*> DivAssign<$rhs_ty> for $lhs_ty {
            fn div_assign(&mut self, rhs: $rhs_ty) {
                self.div_assign_impl($($rhs_deref)? rhs);
            }
        }
        impl_assign_ops!($($rest)*);
    };
}

macro_rules! impl_folding {
    () => {};
    (
        impl <$generic_param:ident : $bound:tt> $trait:ident<_>
            for $self:ty
        {
            fn $method:ident(_) -> _ { _($unit:expr, $op:expr) }
        }
        $($rest:tt)*
    ) => {
        impl<$generic_param: $bound> $trait<Self> for $self {
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = Self>,
            {
                iter.fold($unit, $op)
            }
        }
        impl<'a, $generic_param: $bound> $trait<&'a Self> for $self {
            fn $method<S>(iter: S) -> Self
            where
                S: Iterator<Item = &'a Self>,
            {
                iter.fold($unit, $op)
            }
        }
        impl_folding!($($rest)*);
    };
}

macro_rules! impl_basic_traits {
    () => {};
    (impl<$generic_param:ident : $bound:tt> _ for $self:ty; $($rest:tt)*) => {
        impl<$generic_param: $bound> Default for $self {
            fn default() -> Self { Self::zero() }
        }
        impl<$generic_param: $bound> Hash for $self {
            fn hash<H: Hasher>(&self, state: &mut H) { self.hash_impl(state) }
        }
        impl<$generic_param: $bound> Display for $self {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.display_impl(f)
            }
        }
        impl<$generic_param: $bound> Debug for $self {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.debug_impl(f)
            }
        }
        impl<$generic_param: $bound> Neg for $self {
            type Output = $self;
            fn neg(self) -> Self { self.neg_impl() }
        }
        impl_basic_traits!($($rest)*);
    };
}

impl_bin_ops! {
    for<M: Modulus> <StaticModInt<M>> @ <StaticModInt<M>> -> StaticModInt<M> { self @ }
    for<M: Modulus> <StaticModInt<M>> @ <&'_ StaticModInt<M>> -> StaticModInt<M> { self @ * }
    for<M: Modulus> <&'_ StaticModInt<M>> @ <StaticModInt<M>> -> StaticModInt<M> { self @ }
    for<M: Modulus> <&'_ StaticModInt<M>> @ <&'_ StaticModInt<M>> -> StaticModInt<M> { self @ * }
    for<I: DynamicModIntId> <DynamicModInt<I>> @ <DynamicModInt<I>> -> DynamicModInt<I> { self @ }
    for<I: DynamicModIntId> <DynamicModInt<I>> @ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { self @ * }
    for<I: DynamicModIntId> <&'_ DynamicModInt<I>> @ <DynamicModInt<I>> -> DynamicModInt<I> { self @ }
    for<I: DynamicModIntId> <&'_ DynamicModInt<I>> @ <&'_ DynamicModInt<I>> -> DynamicModInt<I> { self @ * }
}

impl_assign_ops! {
    for<M: Modulus> <StaticModInt<M>> @= <StaticModInt<M>> { self @= }
    for<M: Modulus> <StaticModInt<M>> @= <&'_ StaticModInt<M>> { self @= * }
    for<I: DynamicModIntId> <DynamicModInt<I>> @= <DynamicModInt<I>> { self @= }
    for<I: DynamicModIntId> <DynamicModInt<I>> @= <&'_ DynamicModInt<I>> { self @= * }
}

impl_folding! {
    impl<M: Modulus> Sum<_> for StaticModInt<M> { fn sum(_) -> _ { _(Self::new(0), Add::add)} }
    impl<M: Modulus> Product<_> for StaticModInt<M> { fn product(_) -> _ { _(Self::new(1), Mul::mul)} }
    impl<I: DynamicModIntId> Sum<_> for DynamicModInt<I> { fn sum(_) -> _ { _(Self::new(0), Add::add)} }
    impl<I: DynamicModIntId> Product<_> for DynamicModInt<I> { fn product(_) -> _ { _(Self::new(1), Mul::mul)} }
}

impl_basic_traits! {
    impl<M: Modulus> _ for StaticModInt<M>;
    impl<I: DynamicModIntId> _ for DynamicModInt<I>;
}

pub trait RemEuclidU32 {
    fn rem_euclid_u32(self, n: u32) -> u32;
}

macro_rules! impl_rem_euclid_u32 {
    ( $($ty:ty)* ) => { $(
        impl RemEuclidU32 for $ty {
            fn rem_euclid_u32(self, n: u32) -> u32 {
                self.rem_euclid(n as $ty) as u32
            }
        }
    )* }
}

macro_rules! impl_rem_euclid_u32_small {
    ( $($ty:ty)* ) => { $(
        impl RemEuclidU32 for $ty {
            fn rem_euclid_u32(self, n: u32) -> u32 {
                (self as u32).rem_euclid(n)
            }
        }
    )* }
}

impl_rem_euclid_u32! {
    i64 i128 isize u32 u64 u128 usize
}

impl_rem_euclid_u32_small! {
    i8 i16 u8 u16
}

impl RemEuclidU32 for i32 {
    fn rem_euclid_u32(self, n: u32) -> u32 {
        if self >= 0 {
            (self as u32).rem_euclid(n)
        } else {
            (self as i64).rem_euclid(n as i64) as u32
        }
    }
}

#[test]
fn sanity_check() {
    // assert!(Mod998244353::IS_PRIME);
    // assert!(Mod1000000007::IS_PRIME);

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

#[test]
fn negative() {
    assert_eq!(ModInt998244353::new(-1).get(), 998244352);
}

#[test]
fn fmt() {
    type Mi = ModInt998244353;

    let x = Mi::new(123);
    assert_eq!(format!("{}", x), "123");
    assert_eq!(format!("{:?}", x), "123 (mod 998244353)");
}
