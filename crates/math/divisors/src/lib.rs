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

macro_rules! impl_divisors_unit {
    ( $($ty:ty)* ) => { $(
        impl Divisors for $ty {
            type Output = DivisorsStruct<$ty>;
            fn divisors(self) -> Self::Output {
                Self::Output { below: true, i: 0, n: self }
            }
        }
        impl Iterator for DivisorsStruct<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.below {
                    self.i += 1;
                    while self.i.pow(2) <= self.n && self.n % self.i != 0 {
                        self.i += 1;
                    }
                    if self.i.pow(2) >= self.n {
                        self.below = false;
                    }
                    if self.i.pow(2) <= self.n && self.n % self.i == 0 {
                        return Some(self.i);
                    }
                }
                while self.i > 1 {
                    self.i -= 1;
                    if self.n % self.i == 0 {
                        return Some(self.n / self.i);
                    }
                }
                None
            }
        }
    )* };
}

impl_divisors_unit! { u8 u16 u32 u64 u128 usize }

#[test]
fn test() {
    let n = 10000_u64;
    for i in 0..=n {
        let actual: Vec<_> = i.divisors().collect();
        let expected: Vec<_> = (1..=i).filter(|&j| i % j == 0).collect();
        assert_eq!(actual, expected);
    }
}
