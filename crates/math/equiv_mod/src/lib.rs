//! Chinese remaindering

use gcd_recip::GcdRecip;

/// Chinese remaindering。
///
/// $\\gdef{\\lcm}{\\operatorname{lcm}}$
/// $(r\_0, m\_0)$ と $(r\_1, m\_1)$ に対し、以下を満たす $0\\le x\\lt\\lcm(m\_0, m\_1)$ を求める。
/// - $x\\bmod m\_0 = r\_0$
/// - $x\\bmod m\_1 = r\_1$
///
/// 中国剰余定理から、$\\gcd(m\_0, m\_1)=1$ であれば常に存在する。
/// そうでない場合、高々一つ存在する。
///
/// なお、計算の利便性のため、$\\lcm(m\_0, m\_1)$ も返す。以下の例も参照。
///
/// # Idea
/// $$ \\begin{aligned}
/// (y\\cdot m\_0+r\_0)\\bmod m\_1 &= r\_1 \\\\
/// (y\\cdot m\_0)\\bmod m\_1 &= (r\_1-r\_0)\\bmod m\_1 \\\\
/// \\end{aligned} $$
/// $g = \\gcd(m\_0, m\_1)$ として $(m\_0, m\_1) = (u\_0g, u\_1g)$ とすると、
/// $$ (y\\cdot u\_0g)\\bmod u\_1g = (r\_1-r\_0)\\bmod u\_1g. $$
/// 左辺は $g$ の倍数なので、右辺も $g$ の倍数となる必要がある。
/// よって、$r\_1\\not\\equiv r\_0\\pmod{g}$ であれば解なしとなる。
///
/// 以下、$r\_1\\equiv r\_0\\pmod{g}$ とする。このとき、$r\_1-r\_0$ は $g$ で割り切れ、
/// $$ \\begin{aligned}
/// (y\\cdot u\_0)\\bmod u\_1 &= \\left(\\frac{r\_1-r\_0}{g}\\right)\\bmod u\_1 \\\\
/// y &\\equiv \\left(\\frac{r\_1-r\_0}{g}\\right)\\cdot u\_0^{-1} \\pmod{u\_1}
/// \\end{aligned} $$
/// となる。$\\gcd(u\_0, u\_1)=1$ なので $u\_0^{-1}$ は存在する。
///
/// これを満たす $0\\le y\\lt u\_1$ は一意に定まり、$x = y\\cdot m\_0+r\_0$ を計算すればよい。
/// また、$u\_1 = m\_1/g$ より、$\\lcm(m\_0, m\_1) = m\_0\\cdot u\_1$ となる。
///
/// # Examples
/// ```
/// use nekolib::math::EquivMod;
///
/// assert_eq!((0_i64, 2).equiv_mod((1, 3)), Some((4, 6)));
/// assert_eq!((0_i64, 2).equiv_mod((1, 4)), None);
/// ```
///
/// イテレータと組み合わせてもよい。
/// 条件 $x\\bmod 1 = 0$ が単位元となることに注意。
/// ```
/// use nekolib::math::{EquivMod, Lcm};
///
/// let x = (2_i64..=20)
///     .map(|m| (m - 1, m))
///     .try_fold((0, 1), |x, y| x.equiv_mod(y));
///
/// let lcm = (2_i64..=20).fold(1, |x, y| x.lcm(y));
/// assert_eq!(x, Some((lcm - 1, lcm)));
/// ```
///
/// 簡略版もある。
/// ```
/// use nekolib::math::{EquivModIter, Lcm};
///
/// let x = (2_i64..=20).map(|m| (m - 1, m)).equiv_mod();
/// let lcm = (2_i64..=20).fold(1, |x, y| x.lcm(y));
/// assert_eq!(x, Some((lcm - 1, lcm)));
///
/// assert_eq!(std::iter::empty().equiv_mod(), Some((0_i32, 1)));
/// ```
///
///
/// # Reference
/// - <https://github.com/atcoder/ac-library/blob/master/atcoder/math.hpp>
///
/// ## See also
/// - <https://rsk0315.hatenablog.com/entry/2021/01/18/065720#crt>
pub trait EquivMod: Sized {
    fn equiv_mod(self, other: Self) -> Option<Self>;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl EquivMod for ($t, $t) {
            fn equiv_mod(self, other: Self) -> Option<Self> {
                let ((r0, m0), (r1, m1))
                    = if self.1 >= other.1 { (self, other) } else { (other, self) };
                if m0 % m1 == 0 {
                    return if r0 % m1 == r1 { Some((r0, m0)) } else { None }
                }
                let (g, re) = m0.gcd_recip(m1);
                let u1 = m1 / g;
                let (dr, neg) = if r1 >= r0 { (r1 - r0, false) } else { (r0 - r1, true) };
                if dr % g != 0 {
                    return None
                }
                let x = (if neg { u1 - dr / g % u1 } else { dr / g % u1 }) * re % u1;
                Some((r0 + x * m0, m0 * u1))
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

macro_rules! impl_int {
    ($t:ty) => {
        impl EquivMod for ($t, $t) {
            fn equiv_mod(self, other: Self) -> Option<Self> {
                let ((r0, m0), (r1, m1))
                    = if self.1 >= other.1 { (self, other) } else { (other, self) };
                if m0 % m1 == 0 {
                    return if r0 % m1 == r1 { Some((r0, m0)) } else { None }
                }
                let (g, re) = m0.gcd_recip(m1);
                let u1 = m1 / g;
                if (r1 - r0) % g != 0 {
                    return None
                }
                let x = ((r1 - r0) / g).rem_euclid(u1) * re % u1;
                Some((r0 + x * m0, m0 * u1))
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_int!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);
impl_int!(i8 i16 i32 i64 i128 isize);

#[test]
fn test_uint() {
    let m_max = 150_usize;
    for m0 in 1..=m_max {
        for m1 in 1..=m_max {
            let naive = {
                let mut res = vec![vec![None; m1]; m0];
                for x in (0..m0 * m1).rev() {
                    let r0 = x % m0;
                    let r1 = x % m1;
                    res[r0][r1] = Some(x);
                }
                res
            };
            for r0 in 0..m0 {
                for r1 in 0..m1 {
                    let x = (r0, m0).equiv_mod((r1, m1)).map(|x| x.0);
                    assert_eq!(x, naive[r0][r1]);
                }
            }
        }
    }
}

#[test]
fn test_int() {
    let m_max = 150_isize;
    for m0 in 1..=m_max {
        for m1 in 1..=m_max {
            let naive = {
                let mut res = vec![vec![None; m1 as usize]; m0 as usize];
                for x in (0..m0 * m1).rev() {
                    let r0 = (x % m0) as usize;
                    let r1 = (x % m1) as usize;
                    res[r0][r1] = Some(x);
                }
                res
            };
            for r0 in 0..m0 {
                for r1 in 0..m1 {
                    let x = (r0, m0).equiv_mod((r1, m1)).map(|x| x.0);
                    assert_eq!(x, naive[r0 as usize][r1 as usize]);
                }
            }
        }
    }
}

#[test]
fn test_iter() {
    let x = (2_i64..=20)
        .map(|m| (m - 1, m))
        .try_fold((0, 1), |x, y| x.equiv_mod(y));
    let y = 232792560;

    assert_eq!(x, Some((y - 1, y)));
}

/// Chinese remaindering。
///
/// [`EquivMod`] のイテレータ版。
pub trait EquivModIter<I: Sized> {
    fn equiv_mod(self) -> Option<(I, I)>;
}

macro_rules! impl_iter {
    ($t:ty) => {
        impl<I: Iterator<Item = ($t, $t)>> EquivModIter<$t> for I {
            fn equiv_mod(mut self) -> Option<($t, $t)> {
                self.try_fold((0, 1), |x, y| x.equiv_mod(y))
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_iter!($t);)* };
}

impl_iter!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

#[test]
fn test_iter2() {
    let x = (2_i64..=20).map(|m| (m - 1, m)).equiv_mod();
    let y = 232792560;

    assert_eq!(x, Some((y - 1, y)));
}
