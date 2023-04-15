use std::ops::{Add, AddAssign, Mul, Neg};

// https://atcoder.jp/contests/abc294/editorial/6017
pub trait FractionBisect: Sized + SbInt {
    fn fraction_bisect(
        self,
        pred: impl Fn(Self, Self) -> bool,
    ) -> ((Self, Self), (Self, Self)) {
        let bound = self;

        let fr_neg_infty = (Self::ONE.neg(), Self::ZERO);
        let fr_zero = (Self::ZERO, Self::ONE);
        let ztf = pred(fr_zero.0, fr_zero.1);
        let pred = {
            if !ztf && !Self::SIGNED {
                return (fr_zero, fr_zero);
            }
            if Self::SIGNED && !ztf && !pred(fr_neg_infty.0, fr_neg_infty.1) {
                return (fr_neg_infty, fr_neg_infty);
            }
            move |fr: Fraction<Self>| {
                if ztf { pred(fr.0, fr.1) } else { !pred(fr.0.neg(), fr.1) }
            }
        };

        let mut lower = Fraction::zero();
        let mut upper = Fraction::infty();
        let (small, large) = 'outer: loop {
            let cur = lower + upper;
            if cur.is_deeper(bound) {
                break (lower, upper);
            }

            let tf = pred(cur);
            let (from, to) = if tf { (lower, upper) } else { (upper, lower) };

            let mut lo = Self::ONE;
            let mut hi = lo + Self::ONE;
            while pred(from + to * hi) == tf {
                lo += lo;
                hi += hi;
                if (from + to * lo).is_deeper(bound) {
                    let steps = bound.steps(from.into_inner(), to.into_inner());
                    let front = from + to * steps;

                    let res = if tf { (front, upper) } else { (lower, front) };
                    break 'outer res;
                }
            }

            while lo.lt1(hi) {
                let mid = lo.avg(hi);
                let cur = pred(from + to * mid) == tf;
                *(if cur { &mut lo } else { &mut hi }) = mid;
            }

            let next = from + to * lo;
            *(if tf { &mut lower } else { &mut upper }) = next;
        };

        let (left, right) = if ztf { (small, large) } else { (-large, -small) };
        (left.into_inner(), right.into_inner())
    }
}

impl<I: SbInt> FractionBisect for I {}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Fraction<I>(I, I);

pub trait SbInt:
    Copy
    + Eq
    + PartialOrd<Self>
    + AddAssign<Self>
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + std::fmt::Display
{
    const ZERO: Self;
    const ONE: Self;
    const SIGNED: bool;
    fn lt1(self, other: Self) -> bool;
    fn avg(self, other: Self) -> Self;
    fn abs(self) -> Self;
    fn neg(self) -> Self;
    fn steps(self, from: (Self, Self), to: (Self, Self)) -> Self;
}

impl<I: SbInt> Neg for Fraction<I> {
    type Output = Self;
    fn neg(self) -> Self { self.neg() }
}

macro_rules! impl_uint {
    ( $($ty:ty)* ) => { $(
        impl SbInt for $ty {
            const ZERO: $ty = 0;
            const ONE: $ty = 1;
            const SIGNED: bool = false;
            fn lt1(self, other: Self) -> bool { self + 1 < other }
            fn avg(self, other: Self) -> Self {
                self + (other - self) / 2
            }
            fn abs(self) -> Self { self }
            fn neg(self) -> Self { self.wrapping_neg() } // not to be called
            fn steps(self, from: (Self, Self), to: (Self, Self)) -> Self {
                if to.0 == 0 {
                    (self - from.1) / to.1
                } else if to.1 == 0 {
                    (self - from.0) / to.0
                } else {
                    ((self - from.0) / to.0).min((self - from.1) / to.1)
                }
            }
        }
    )* }
}

impl_uint! { u8 u16 u32 u64 u128 usize }

macro_rules! impl_int {
    ( $($ty:ty)* ) => { $(
        impl SbInt for $ty {
            const ZERO: $ty = 0;
            const ONE: $ty = 1;
            const SIGNED: bool = true;
            fn lt1(self, other: Self) -> bool { self + 1 < other }
            fn avg(self, other: Self) -> Self {
                self + (other - self) / 2
            }
            fn abs(self) -> Self { self.abs() }
            fn neg(self) -> Self { -self}
            fn steps(self, from: (Self, Self), to: (Self, Self)) -> Self {
                if to.0 == 0 {
                    (self - from.1) / to.1
                } else if to.1 == 0 {
                    (self - from.0) / to.0
                } else {
                    ((self - from.0) / to.0).min((self - from.1) / to.1)
                }
            }
        }
    )* }
}

impl_int! { i8 i16 i32 i64 i128 isize }

impl<I: SbInt> Fraction<I> {
    fn zero() -> Self { Self(I::ZERO, I::ONE) }
    fn infty() -> Self { Self(I::ONE, I::ZERO) }
}

impl<I: SbInt> Mul<I> for Fraction<I> {
    type Output = Self;
    fn mul(self, a: I) -> Self { Self(self.0 * a, self.1 * a) }
}

impl<I: SbInt> Add<Fraction<I>> for Fraction<I> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        // mediant
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<I: SbInt> Fraction<I> {
    fn is_deeper(self, bound: I) -> bool { self.1.abs() > bound }
    fn neg(self) -> Self { Self(self.0.neg(), self.1) }
    fn into_inner(self) -> (I, I) { (self.0, self.1) }
}

#[test]
fn sanity_check() {
    let sqrt3 = 5000_u64.fraction_bisect(|x, y| x * x <= 3 * y * y);
    assert_eq!(sqrt3, ((3691, 2131), (5042, 2911)));

    assert_eq!(10_u64.fraction_bisect(|_, _| false), ((0, 1), (0, 1)));
    assert_eq!(10_i64.fraction_bisect(|_, _| false), ((-1, 0), (-1, 0)));

    let neg_sqrt3 = 5000_i64.fraction_bisect(|x, y| x < 0 && x * x > 3 * y * y);
    assert_eq!(neg_sqrt3, ((-5042, 2911), (-3691, 2131)));

    let lt = 5000_i64.fraction_bisect(|x, y| 5 * x < 2 * y);
    assert_eq!(lt, ((1999, 4998), (2, 5)));
    let le = 5000_i64.fraction_bisect(|x, y| 5 * x <= 2 * y);
    assert_eq!(le, ((2, 5), (1999, 4997)));
}

#[test]
fn sqrt() {
    let sqrt3 = 10_u128.pow(18).fraction_bisect(|x, y| x * x <= 3 * y * y);
    let sqrt4 = 10_u128.pow(18).fraction_bisect(|x, y| x * x <= 4 * y * y);

    assert_eq!(sqrt3.0, (734231055024833855, 423908497265970753));
    assert_eq!(sqrt3.1, (1002978273411373057, 579069776145402304));

    assert_eq!(sqrt4.0, (2, 1));
    assert_eq!(sqrt4.1, (999999999999999999, 499999999999999999));
}

#[test]
fn improper_fraction() {
    let x = 6_u32.fraction_bisect(|x, y| x * 5 <= y * 13);
    assert_eq!(x, ((13, 5), (8, 3)));
}
