//! 約数列挙。

/// 約数列挙。
///
/// # Complexity
/// $O(\\sqrt{n})$ time, $O(1)$ space.
///
/// # Examples
/// ```
/// use nekolib::math::Divisors;
///
/// let div: Vec<_> = 60_u64.divisors().collect();
/// assert_eq!(div, [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
/// ```
pub trait Divisors {
    type Output;
    fn divisors(self) -> Self::Output;
}

pub struct DivisorsStruct<I> {
    below: bool,
    i: I,
    n: I,
}

macro_rules! impl_divisors_uint {
    ( $($ty:ty)* ) => { $(
        impl Divisors for $ty {
            type Output = DivisorsStruct<$ty>;
            fn divisors(self) -> Self::Output {
                Self::Output { below: true, i: 0, n: self }
            }
        }
        impl DivisorsStruct<$ty> {
            fn increment(&mut self) -> Option<$ty> {
                if !self.below {
                    return None;
                }
                loop {
                    self.i += 1;
                    let ii = if let Some(ii) = self.i.checked_pow(2) {
                        ii
                    } else {
                        self.below = false;
                        return None;
                    };
                    if ii >= self.n {
                        self.below = false;
                        if ii > self.n {
                            return None;
                        }
                    }
                    if self.n % self.i == 0 {
                        return Some(self.i);
                    }
                }
            }
            fn decrement(&mut self) -> Option<$ty> {
                while self.i > 1 {
                    self.i -= 1;
                    if self.n % self.i == 0 {
                        return Some(self.n / self.i);
                    }
                }
                None
            }
        }
        impl Iterator for DivisorsStruct<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                self.increment().or_else(|| self.decrement())
            }
        }
    )* };
}

impl_divisors_uint! { u8 u16 u32 u64 u128 usize }

#[test]
fn test() {
    let n = 10000_u64;
    for i in 0..=n {
        let actual: Vec<_> = i.divisors().collect();
        let expected: Vec<_> = (1..=i).filter(|&j| i % j == 0).collect();
        assert_eq!(actual, expected);
    }
}

#[test]
fn overflow() {
    for i in (1_u32..=1000)
        .flat_map(|i| [i.wrapping_neg(), 2_u32.pow(16) * (2_u32.pow(16) - i)])
    {
        let actual: Vec<_> = i.divisors().collect();
        let expected: Vec<_> =
            (i as u64).divisors().map(|d| d as u32).collect();
        assert_eq!(actual, expected);
    }
}

#[test]
fn overflow_exhaustive() {
    for i in u8::MIN..=u8::MAX {
        let actual: Vec<_> = i.divisors().collect();
        let expected: Vec<_> = (i as u32).divisors().map(|d| d as u8).collect();
        assert_eq!(actual, expected);
    }
    for i in u16::MIN..=u16::MAX {
        let actual: Vec<_> = i.divisors().collect();
        let expected: Vec<_> =
            (i as u32).divisors().map(|d| d as u16).collect();
        assert_eq!(actual, expected);
    }
}
