pub trait Digits: Sized {
    fn digits(self, base: Self) -> DigitsIter<Self>;
}

pub struct DigitsIter<I> {
    x: I,
    base: I,
}

impl<I> DigitsIter<I> {
    pub fn new(x: I, base: I) -> Self { Self { x, base } }
}

macro_rules! impl_uint {
    ( $($ty:ty)* ) => { $(
        impl Digits for $ty {
            fn digits(self, base: Self) -> DigitsIter<Self> {
                DigitsIter::new(self, base)
            }
        }
        impl Iterator for DigitsIter<$ty> {
            type Item = $ty;
            fn next(&mut self) -> Option<$ty> {
                if self.x == 0 {
                    return None;
                }

                let res = self.x % self.base;
                self.x /= self.base;
                Some(res)
            }
        }
    )* }
}

impl_uint! { u8 u16 u32 u64 u128 usize }

#[test]
fn sanity_check() {
    let a: Vec<_> = 1234_u32.digits(10).collect();
    assert_eq!(a, [4, 3, 2, 1]);

    let a: Vec<_> = 0_u32.digits(10).collect();
    assert!(a.is_empty());

    let a: Vec<_> = 13_u32.digits(2).collect();
    assert_eq!(a, [1, 0, 1, 1]);
}
