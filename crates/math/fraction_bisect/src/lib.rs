use std::ops::{Add, AddAssign, Mul};

// https://atcoder.jp/contests/abc294/editorial/6017
pub trait FractionBisect: Sized + SbInt {
    fn fraction_bisect(
        self,
        pred: impl Fn(Self, Self) -> bool,
    ) -> (Self, Self) {
        let bound = self;
        let pred = |fr: Fraction<_>| pred(fr.0, fr.1);

        let mut lower = Fraction::zero();
        let mut upper = Fraction::infty();
        loop {
            let cur = lower + upper;
            if cur.is_deeper(bound) {
                return upper.into_inner();
            }

            let tf = pred(cur);
            let (from, to) = if tf { (lower, upper) } else { (upper, lower) };

            let mut lo = Self::ONE;
            let mut hi = lo + Self::ONE;
            while pred(from + to * hi) == tf {
                lo += lo;
                hi += hi;
                if (from + to * lo).is_deeper(bound) {
                    return to.into_inner();
                }
            }

            while lo.lt1(hi) {
                let mid = lo.avg(hi);
                let cur = pred(from + to * mid) == tf;
                *(if cur { &mut lo } else { &mut hi }) = mid;
            }

            let next = from + to * lo;
            *(if tf { &mut lower } else { &mut upper }) = next;
        }
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
{
    const ZERO: Self;
    const ONE: Self;
    fn lt1(self, other: Self) -> bool;
    fn avg(self, other: Self) -> Self;
}

macro_rules! impl_uint {
    ( $($ty:ty)* ) => { $(
        impl SbInt for $ty {
            const ZERO: $ty = 0;
            const ONE: $ty = 1;
            fn lt1(self, other: Self) -> bool { self + 1 < other }
            fn avg(self, other: Self) -> Self {
                self + (other - self) / 2
            }
        }
    )* }
}

impl_uint! { u8 u16 u32 u64 u128 usize }

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
    fn is_deeper(self, bound: I) -> bool { self.0 > bound || self.1 > bound }
    fn into_inner(self) -> (I, I) { (self.0, self.1) }
}

#[test]
fn sanity_check() {
    let sqrt3 = 5000_u64.fraction_bisect(|x, y| x * x <= 3 * y * y);
    // assert_eq!(sqrt3, (5042, 2911));
    assert_eq!(sqrt3, (1351, 780));
}
