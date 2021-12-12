//! ローリングハッシュに関する wrapper クラス。

use std::fmt::Debug;

use binop::{Associative, Identity, Magma};

/// 文字列連結をローリングハッシュで行う演算を持つ。
///
/// # Examples
/// ```
/// use nekolib::math::ModInt;
/// use nekolib::traits::Magma;
/// use nekolib::utils::OpRollHash;
///
/// let op_rh = OpRollHash::<998244353>::default();
/// let value_of = |s| op_rh.value_from(s);
/// let op = |x, y| op_rh.op(x, y);
///
/// let abr = value_of("abr");
/// let a = value_of("a");
/// let abra = value_of("abra");
/// assert_eq!(op(abr, a), abra);
///
/// let s = "abracadabra";
/// assert_eq!(value_of(&s[0..4]), abra);
/// assert_eq!(value_of(&s[7..11]), abra);
/// assert_ne!(value_of(&s[1..5]), abra);
/// assert_eq!(value_of(s), op(op(abra, value_of("cad")), abra));
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
