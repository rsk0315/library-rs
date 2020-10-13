//! ローリングハッシュに関する wrapper クラス。

use std::fmt::Debug;
use std::ops::{Add, Mul};

use additive::Zero;
use assoc_val::*;
use binop::*;
use multiplicative::One;

/// 文字列連結をローリングハッシュで行う演算を持つ。
///
/// # Examples
/// ```
/// use nekolib::traits::binop::*;
/// use nekolib::traits::AssocVal;
/// use nekolib::utils::OpRollHash;
/// use nekolib::utils::ModInt;
/// use nekolib::{impl_assoc_val, impl_mod_int};
///
/// impl_mod_int! { Mod1e9p7 => 1_000_000_007_i64 }
/// type Mi = ModInt<Mod1e9p7>;
/// impl_assoc_val! { Base<Mi> => Mi::from(123) }
///
/// let val = |s| OpRollHash::<Mi, Base>::val_from(s);
/// let op = |x, y| OpRollHash::<Mi, Base>::op(x, y);
///
/// let abr = val("abr");
/// let a = val("a");
/// let abra = val("abra");
/// assert_eq!(op(abr, a), abra);
///
/// let s = "abracadabra";
/// assert_eq!(val(&s[0..4]), abra);
/// assert_eq!(val(&s[7..11]), abra);
/// assert_ne!(val(&s[1..5]), abra);
/// assert_eq!(val(s), op(op(abra, val("cad")), abra));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpRollHash<T, B>
where
    T: Copy + Eq,
    B: AssocVal<T>,
{
    _tb: std::marker::PhantomData<(T, B)>,
}

impl<T, B> Magma for OpRollHash<T, B>
where
    T: Copy + Eq + Add<Output = T> + Mul<Output = T> + Zero,
    B: AssocVal<T>,
{
    type Set = (T, T);
    fn op((hx, lx): Self::Set, (hy, ly): Self::Set) -> Self::Set {
        (hx * ly + hy, lx * ly)
    }
}

impl<T, B> Identity for OpRollHash<T, B>
where
    T: Copy + Eq + Add<Output = T> + Mul<Output = T> + Zero + One,
    B: AssocVal<T>,
{
    fn id() -> Self::Set {
        (T::zero(), T::one())
    }
}

impl<T, B> Associative for OpRollHash<T, B>
where
    T: Copy + Eq + Add<Output = T> + Mul<Output = T> + Zero,
    B: AssocVal<T>,
{
}

impl<T, B> OpRollHash<T, B>
where
    T: Copy + Eq + Add<Output = T> + Mul<Output = T> + Zero + One + From<u8>,
    B: AssocVal<T>,
{
    pub fn val_from(s: &str) -> <Self as Magma>::Set {
        s.bytes()
            .fold(Self::id(), |acc, x| Self::op(acc, (T::from(x), B::get())))
    }
}
