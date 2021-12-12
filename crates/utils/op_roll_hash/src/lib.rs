//! ローリングハッシュに関する wrapper クラス。

use std::fmt::Debug;

use binop::{Associative, Identity, Magma};

/// 文字列連結をローリングハッシュで行う演算を持つ。
///
/// # Examples
/// ```
/// use nekolib::math::ModInt;
/// use nekolib::traits::{AssocVal, Magma};
/// use nekolib::utils::OpRollHash;
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
pub enum OpRollHash<const B: u64> {
    OpRollHashV,
}
pub use OpRollHash::OpRollHashV;

impl<const B: u64> Default for OpRollHash<B> {
    fn default() -> Self { OpRollHashV }
}

impl<const B: u64> Magma for OpRollHash<B> {
    type Set = (u64, u64);
    fn op(&self, (hx, lx): Self::Set, (hy, ly): Self::Set) -> Self::Set {
        ((hx * ly + hy) % B, (lx * ly) % B)
    }
}

impl<const B: u64> Identity for OpRollHash<B> {
    fn id(&self) -> Self::Set { (0, 1) }
}

impl<const B: u64> Associative for OpRollHash<B> {}

impl<const B: u64> OpRollHash<B> {
    #[must_use]
    pub fn value_of(&self, s: &str) -> <Self as Magma>::Set {
        s.chars().fold((0, 0), |acc, x| self.op(acc, (x as u64, B)))
    }
}
