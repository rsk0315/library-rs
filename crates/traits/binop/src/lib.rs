//! 代数的構造に関するトレイトたちです。
//!
//! データ構造を実装する際に使うことを目的とします。

/// マグマ。
///
/// 集合 $M$ と二項演算 $\\circ$ のペア $(M, \\circ)$ であり、次の性質を満たす。
/// $$ x, y \\in M \\implies x \\circ y \\in M. $$
///
/// # Examples
/// ```
/// use nekolib::traits::Magma;
/// use nekolib::utils::OpMin;
///
/// assert_eq!(3, OpMin::op(3, 4));
/// ```
pub trait Magma {
    /// 集合 $M$ に対応する型。
    type Set: Eq;
    /// $x \\circ y$ を返す。
    fn op(x: Self::Set, y: Self::Set) -> Self::Set;
}

/// 結合法則を満たす。
///
/// 二項演算 $\\circ: M \\times M \\to M$ が結合法則を満たすことを示す。
/// $$ x, y, z \\in M \\implies (x \\circ y) \\circ z = x \\circ (y \\circ z). $$
/// # Examples
/// ```
/// use nekolib::traits::{Associative, Magma};
/// use nekolib::utils::OpMin;
///
/// let (x, y, z) = (2, 3, 4);
/// assert_eq!(
///     OpMin::op(OpMin::op(x, y), z),
///     OpMin::op(x, OpMin::op(y, z)),
/// );
/// ```
pub trait Associative: Magma {}

/// 単位元を持つ。
///
/// 二項演算 $\\circ: M \\times M \\to M$ が単位元を持つことを示す。
/// $$ x \\in M \\implies x \\circ e = e \\circ x = e. $$
///
/// # Examples
/// ```
/// use nekolib::traits::{Identity, Magma};
/// use nekolib::utils::OpMin;
///
/// let x = 3;
/// assert_eq!(OpMin::<i32>::id(), std::i32::MAX);
/// assert_eq!(OpMin::op(x, OpMin::id()), x);
/// ```
pub trait Identity: Magma {
    /// 単位元を返す。
    fn id() -> Self::Set;
}

/// 交換法則を満たす。
///
/// 二項演算 $\\circ: M \\times M \\to M$ が交換法則を満たすことを示す。
/// $$ x, y \\in M \\implies x \\circ y = y \\circ x. $$
/// 交換法則を満たさない演算の例としては、文字列結合や線形関数の合成、行列積などが挙げられる。
///
/// # Examples
/// ```
/// use nekolib::traits::{Commutative, Magma};
/// use nekolib::utils::OpMin;
///
/// let (x, y) = (3, 4);
/// assert_eq!(OpMin::op(x, y), OpMin::op(y, x));
/// ```
pub trait Commutative: Magma {}

/// 逆元を持つ要素が存在する。
///
/// 二項演算 $\\circ: M \\times M \\to M$ が、一部の要素を除いて逆元を持つことを示す。
///
/// 体の乗法においては $0$ を除いて逆元を持つことが要請されるため必要かなと思った。
/// もっといい設計はある気がする。
pub trait PartialRecip: Magma {
    fn partial_recip(x: Self::Set) -> Option<Self::Set>;
}

/// 逆元が常に存在する。
///
/// 二項演算 $\\circ: M \\times M \\to M$ が、常に逆元を持つことを示す。
/// $$ x \\in M \\implies {}^\\exists a \\in M: x \\circ a = a \\circ x = e. $$
/// この $a$ を $x^{-1}$ と書く。
///
/// # Examples
/// ```
/// use nekolib::traits::{Magma, Monoid, Recip};
/// use nekolib::utils::OpAdd;
///
/// let x = 3;
/// let y = OpAdd::recip(x);
/// assert_eq!(OpAdd::op(x, y), 0);
/// ```
pub trait Recip: PartialRecip {
    fn recip(x: Self::Set) -> Self::Set { Self::partial_recip(x).unwrap() }
}

/// 分配法則を満たす。
///
/// 乗法 $\\ast: M \\times M \\to M$ は、加法 $\\circ: M \\times M \\to M$ について
/// 分配法則を満たすことを示す。
/// $$ \\begin{aligned}
/// x, y, z \\in R &\\implies x \\ast (y \\circ z) = (x \\ast y) \\circ (x \\ast z), \\\\
/// x, y, z \\in R &\\implies (y \\circ z) \\ast x = (y \\ast x) \\circ (z \\ast x).
/// \\end{aligned} $$
/// 加法は型引数 `A` として指定される。
///
/// # Examples
/// ```
/// use nekolib::traits::{Commutative, Magma};
/// use nekolib::utils::{OpAdd, OpMul};
///
/// let (x, y, z) = (3, 4, 5);
/// assert_eq!(
///     OpMul::op(x, OpAdd::op(y, z)),
///     OpAdd::op(OpMul::op(x, y), OpMul::op(x, z))
/// );
/// ```
pub trait Distributive<A: Magma> {}

/// 半群。
///
/// マグマ $(M, \\circ)$ であり、結合法則を満たす。
pub trait Semigroup: Associative + Magma {}
impl<G: Associative + Magma> Semigroup for G {}

/// モノイド。
///
/// 半群 $(M, \\circ)$ であり、単位元 $e \\in M$ を持つ。
pub trait Monoid: Identity + Semigroup {}
impl<G: Identity + Semigroup> Monoid for G {}

/// 可換モノイド。
///
/// モノイド $(M, \\circ, e)$ であり、交換法則を満たす。
pub trait CommutativeMonoid: Commutative + Monoid {}
impl<G: Commutative + Monoid> CommutativeMonoid for G {}

/// 群。
///
/// モノイド $(M, \\circ, e)$ であり、逆元を持つ。
pub trait Group: Monoid + Recip {}
impl<G: Monoid + Recip> Group for G {}

/// 可換群。
///
/// 群 $(M, \\circ, e)$ であり、交換法則を満たす。
pub trait CommutativeGroup: Commutative + Monoid + Recip {}
impl<G: Commutative + Monoid + Recip> CommutativeGroup for G {}

/// 環。
///
/// 集合 $R$ と二つの二項演算 $\\circ$, $\\ast$ の組 $(R, \\circ, \\ast)$ であり、次の性質を満たす。
/// - $(R, \\circ, 0)$ は可換群をなす。
/// - $(R, \\ast, 1)$ はモノイドをなす。
/// - 乗法 $\\ast$ は加法 $\\circ$ について分配法則を満たす。
pub trait Ring {
    /// 集合 $R$ に対応する型。
    type Set: Eq;
    /// 可換群 $(R, \\circ, 0)$ に対応する型。
    type Additive: CommutativeGroup<Set = Self::Set>;
    /// モノイド $(R, \\ast, 1)$ に対応する型。
    type Multiplicative: Monoid<Set = Self::Set> + Distributive<Self::Additive>;

    /// 和 $x \\circ y$ を返す。
    fn add(x: Self::Set, y: Self::Set) -> Self::Set { Self::Additive::op(x, y) }
    /// 加法 $\\circ$ の単位元 $0$ を返す。
    #[must_use]
    fn zero() -> Self::Set { Self::Additive::id() }
    /// 加法 $\\circ$ に関する $x$ の逆元 $-x$ を返す。
    fn neg(x: Self::Set) -> Self::Set { Self::Additive::recip(x) }
    /// 積 $x \\ast y$ を返す。
    fn mul(x: Self::Set, y: Self::Set) -> Self::Set {
        Self::Multiplicative::op(x, y)
    }
    /// 乗法 $\\ast$ の単位元 $1$ を返す。
    #[must_use]
    fn one() -> Self::Set { Self::Multiplicative::id() }
}

/// 可換環。
///
/// 環 $(R, \\circ, \\ast, 0, 1)$ であり、$(R, \\ast, 1)$ は可換モノイドをなす。
pub trait CommutativeRing: Ring
where
    Self::Multiplicative: Commutative,
{
}

/// 体。
///
/// 環 $(R, \\circ, \\ast, 0, 1)$ であり、$(R \\setminus \\{0\\}, \\ast, 1)$ は群をなす。
pub trait Field: Ring
where
    Self::Multiplicative: PartialRecip,
{
    /// 乗法 $\\ast$ における関する $x$ の逆元 $x^{-1}$ を返す。
    fn recip(x: Self::Set) -> Self::Set {
        if x == Self::Additive::id() {
            panic!("zero element does not have multiplicative inverse");
        } else {
            Self::Multiplicative::partial_recip(x).unwrap()
        }
    }
}
