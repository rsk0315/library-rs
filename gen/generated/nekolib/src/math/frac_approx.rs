use std::ops::{Add, AddAssign, Mul, Neg};

pub struct FracApproxIter<I, F> {
    lower: Fraction<I>,
    upper: Fraction<I>,
    bound: Option<I>,
    pred: F,
}

impl<I: SbInt, F: Fn(I, I) -> bool> FracApproxIter<I, F> {
    fn new(pred: F, bound: Option<I>) -> Self {
        let (lower, upper) = if !I::SIGNED {
            (I::lowest_frac(), I::highest_frac())
        } else if pred(I::ZERO, I::ONE) {
            (I::zero_frac(), I::highest_frac())
        } else {
            (I::lowest_frac(), I::zero_frac())
        };
        let (lower, upper) = (lower.into(), upper.into());
        Self { lower, upper, bound, pred }
    }

    fn iter_return(&mut self, q: I, tf: bool) -> (ApproxFrac<I>, I) {
        (ApproxFrac::from(((self.lower, self.upper), tf)), q)
    }
}

impl<I: SbInt, F: Fn(I, I) -> bool> Iterator for FracApproxIter<I, F> {
    type Item = (ApproxFrac<I>, I);
    fn next(&mut self) -> Option<Self::Item> {
        let Self { lower, upper, bound, pred } = self;
        let pred = |fr: Fraction<I>| {
            let (lower, upper) = fr.into();
            pred(lower, upper)
        };
        let median = *lower + *upper;

        if median.is_deeper(*bound) {
            return None;
        }

        let tf = pred(median);
        let (from, to) = if tf { (*lower, *upper) } else { (*upper, *lower) };

        let mut lo = I::ONE;
        let mut hi = lo + I::ONE;
        while pred(from + to * hi) == tf {
            lo += lo;
            hi += hi;
            if (from + to * lo).is_deeper(*bound) {
                let bound =
                    if let Some(bound) = *bound { bound } else { continue };
                let steps = bound.steps(from.into_inner(), to.into_inner());
                let front = from + to * steps;
                *(if tf { lower } else { upper }) = front;
                return Some(self.iter_return(steps, tf));
            }
        }

        while lo.lt1(hi) {
            let mid = lo.avg(hi);
            let cur = pred(from + to * mid) == tf;
            *(if cur { &mut lo } else { &mut hi }) = mid;
        }

        *(if tf { lower } else { upper }) = from + to * lo;
        Some(self.iter_return(lo, tf))
    }
}

pub enum ApproxFrac<I> {
    Lower((I, I)),
    Upper((I, I)),
}
use ApproxFrac::{Lower, Upper};

impl<I: std::fmt::Display> std::fmt::Debug for ApproxFrac<I> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, (num, den)) = match self {
            Lower(x) => ("Lower", x),
            Upper(x) => ("Upper", x),
        };
        fmt.debug_tuple(name).field(&format!("{}/{}", num, den)).finish()
    }
}

impl<I> ApproxFrac<I> {
    pub fn into_inner(self) -> (I, I) {
        match self {
            Lower(x) | Upper(x) => x,
        }
    }
}

impl<I: SbInt> From<((Fraction<I>, Fraction<I>), bool)> for ApproxFrac<I> {
    fn from(((lower, upper), tf): ((Fraction<I>, Fraction<I>), bool)) -> Self {
        if tf { Lower(lower.into()) } else { Upper(upper.into()) }
    }
}

pub trait FracApprox<I, F> {
    fn frac_approx_iter(self) -> FracApproxIter<I, F>;
    fn frac_approx_iter_bound(self, bound: I) -> FracApproxIter<I, F>;
}

impl<I: SbInt, F: Fn(I, I) -> bool> FracApprox<I, F> for F {
    fn frac_approx_iter(self) -> FracApproxIter<I, F> {
        FracApproxIter::new(self, None)
    }
    fn frac_approx_iter_bound(self, bound: I) -> FracApproxIter<I, F> {
        FracApproxIter::new(self, Some(bound))
    }
}

pub trait SbInt:
    Copy
    + Eq
    + PartialOrd<Self>
    + AddAssign<Self>
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + std::fmt::Display
    + std::fmt::Debug
{
    const ZERO: Self;
    const ONE: Self;
    const SIGNED: bool;
    fn lt1(self, other: Self) -> bool;
    fn avg(self, other: Self) -> Self;
    fn abs(self) -> Self;
    fn neg(self) -> Self;
    fn steps(self, from: (Self, Self), to: (Self, Self)) -> Self;
    fn lowest_frac() -> (Self, Self);
    fn highest_frac() -> (Self, Self);
    fn zero_frac() -> (Self, Self);
}

#[derive(Clone, Copy, Eq, PartialEq, std::fmt::Debug)]
struct Fraction<I>(I, I);

impl<I: SbInt> From<(I, I)> for Fraction<I> {
    fn from((num, den): (I, I)) -> Self { Self(num, den) }
}

impl<I: SbInt> From<Fraction<I>> for (I, I) {
    fn from(frac: Fraction<I>) -> Self { (frac.0, frac.1) }
}

impl<I: SbInt> Neg for Fraction<I> {
    type Output = Self;
    fn neg(self) -> Self { self.neg() }
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
    fn is_deeper(self, bound: Option<I>) -> bool {
        bound.into_iter().any(|b| self.1 > b)
    }
    fn neg(self) -> Self { Self(self.0.neg(), self.1) }
    fn into_inner(self) -> (I, I) { (self.0, self.1) }
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
            fn lowest_frac() -> (Self, Self) { (0, 1) }
            fn highest_frac() -> (Self, Self) { (1, 0) }
            fn zero_frac() -> (Self, Self) { (0, 1) }
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
            fn lowest_frac() -> (Self, Self) { (-1, 0) }
            fn highest_frac() -> (Self, Self) { (1, 0) }
            fn zero_frac() -> (Self, Self) { (0, 1) }
        }
    )* }
}

impl_int! { i8 i16 i32 i64 i128 isize }

#[test]
fn sanity_check() {
    let sqrt2 = |p: i128, q| p * p <= 2 * q * q;
    for ap in sqrt2.frac_approx_iter().take(30) {
        eprintln!("{ap:?}");
    }

    let frac = |p: i128, q| 113 * p <= 355 * q;
    for ap in frac.frac_approx_iter_bound(113) {
        eprintln!("{ap:?}");
    }
}
