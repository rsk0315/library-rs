//! 位数。

use carmichael_lambda::CarmichaelLambda;
use factors::Factors;
use gcd::Gcd;
use mod_pow::ModPow;

/// 位数。
///
/// 法を $n$ として $a, a^2, \\dots, a^m$ が互いに異なり、かつ $a^m \\equiv 1$ である
/// $m$ が存在すれば、それを返す。
///
/// $0\\le a\\lt n$ とする。
///
/// # Complexity
/// $\\lambda(n)$ に対する素因数列挙にかかる時間に加え、各素因数に対して
/// $O(\\log(\\lambda(n)))$ 時間。試し割り法では $O(\\sqrt{n})$ 時間。
///
/// # Examples
/// ```
/// use nekolib::math::ModOrd;
///
/// assert_eq!(7_u64.mod_ord(10), Some(4));
/// assert_eq!(1_u64.mod_ord(3), Some(1));
/// assert_eq!(22_u64.mod_ord(30), None);
/// ```
///
/// # Suggestions
/// [`dlog`] と同様、$\\lambda$ 関数と素因数列挙に関して引数で渡したいかも。
///
/// [`dlog`]: fn.dlog.html
pub trait ModOrd: Sized {
    fn mod_ord(self, other: Self) -> Option<Self>;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl ModOrd for $t {
            fn mod_ord(self, n: Self) -> Option<Self> {
                assert_ne!(n, 0, "modulo must be positive");
                let a = match self {
                    0 => return None,
                    1 => return Some(1),
                    _ => self,
                };

                let g = a.gcd(n);
                if g != 1 {
                    return None;
                }

                let mut q = n.carmichael_lambda();
                for e in q.factors_dup() {
                    if a.mod_pow(q / e, n) == 1 {
                        q /= e;
                    }
                }
                (a.mod_pow(q, n) == 1).then(|| q)
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

#[test]
fn test() {
    let n_max = 500_u64;

    for n in 2..=n_max {
        for a in 0..n {
            let actual = a.mod_ord(n);
            let mut x = 1;
            let expected = (1..=n).find_map(|i| {
                x = x * a % n;
                if x == 1 {
                    Some(i)
                } else {
                    None
                }
            });
            eprintln!("{:?}", (a, n));
            assert_eq!(actual, expected);
        }
    }
}
