//! 約数列挙。

/// 約数列挙。
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// # Examples
/// ```
/// use nekolib::math::Divisors;
///
/// let div: Vec<_> = 60_u64.divisors().collect();
/// assert_eq!(div, [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]);
/// ```
pub trait Divisors: Sized {
    // impl Iterator<Item = Self> + DoubleEndedIterator
    fn divisors(
        self,
    ) -> std::iter::Chain<
        std::vec::IntoIter<Self>,
        std::iter::Rev<std::vec::IntoIter<Self>>,
    >;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl Divisors for $t {
            fn divisors(self) ->
                std::iter::Chain<
                    std::vec::IntoIter<$t>,
                    std::iter::Rev<std::vec::IntoIter<$t>>
                >
            {
                let n = self;
                let mut former = vec![];
                let mut latter = vec![];
                for i in (1..=n)
                    .take_while(|&i| i * i <= n)
                    .filter(|&i| n % i == 0)
                {
                    former.push(i);
                    if i * i < n {
                        latter.push(n / i);
                    }
                }
                former.into_iter().chain(latter.into_iter().rev())
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

#[test]
fn test() {
    let n = 1000_u64;
    for i in 1..=n {
        let actual: Vec<_> = i.divisors().collect();
        let expected: Vec<_> = (1..=i).filter(|&j| i % j == 0).collect();
        assert_eq!(actual, expected);
    }
}
