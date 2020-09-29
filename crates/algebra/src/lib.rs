//! 代数的構造に関するトレイトたちです。
//! データ構造を実装する際に使うことを目的とします。

/// マグマ。集合 $M$ と二項演算 $\\circ$ のペア $(M, \\circ)$ であり、次の性質を満たす。
/// $$ x, y \\in M \\implies x \\circ y \\in M. $$
///
/// # 例
///
/// ```
/// use algebra::Magma;
///
/// struct Min<T> {
///     _t: std::marker::PhantomData<T>,
/// }
///
/// impl<T: Ord> Magma for Min<T> {
///     type Set = T;
///     fn op(x: Self::Set, y: Self::Set) -> Self::Set {
///         x.min(y)
///     }
/// }
///
/// assert_eq!(3, Min::<i32>::op(3, 4));
/// ```
pub trait Magma {
    /// 集合 $M$ に対応する型。
    type Set: Eq;
    /// $x \\circ y$ を返す。
    fn op(x: Self::Set, y: Self::Set) -> Self::Set;
}

/// 二項演算 $\\circ: M \\times M \\to M$ が結合法則を満たすことを示す。
/// $$ x, y, z \\in M \\implies (x \\circ y) \\circ z = x \\circ (y \\circ z). $$
/// # 例
///
/// ```
/// use algebra::{Associative, Magma};
///
/// struct Min<T> {
///     _t: std::marker::PhantomData<T>,
/// }
///
/// impl<T: Ord> Magma for Min<T> {
///     type Set = T;
///     fn op(x: Self::Set, y: Self::Set) -> Self::Set {
///         x.min(y)
///     }
/// }
/// impl<T: Ord> Associative for Min<T> {}
///
/// assert_eq!(
///     Min::<i32>::op(Min::<i32>::op(2, 3), 4),
///     Min::<i32>::op(2, Min::<i32>::op(3, 4)),
/// );
/// ```
pub trait Associative: Magma {}

/// 二項演算 $\\circ: M \\times M \\to M$ が単位元を持つことを示す。
/// $$ x \\in M \\implies x \\circ e = e \\circ x = e. $$
/// # 例
///
/// ```
/// use algebra::{Identity, Magma};
///
/// struct Min<T> {
///     _t: std::marker::PhantomData<T>,
/// }
///
/// impl<T: Ord> Magma for Min<T> {
///     type Set = T;
///     fn op(x: Self::Set, y: Self::Set) -> Self::Set {
///         x.min(y)
///     }
/// }
/// impl Identity for Min<i32> {
///     fn id() -> Self::Set {
///         std::i32::MAX
///     }
/// }
///
/// assert_eq!(3, Min::<i32>::op(3, Min::<i32>::id()));
/// ```
pub trait Identity: Magma {
    /// 単位元を返す。
    fn id() -> Self::Set;
}

/// 二項演算 $\\circ: M \\times M \\to M$ が交換法則を満たすことを示す。
/// $$ x, y \\in M \\implies x \\circ y = y \\circ x. $$
/// 交換法則を満たさない演算の例としては、文字列結合や線形関数の合成、行列積などが挙げられる。
/// # 例
///
/// ```
/// use algebra::{Commutative, Magma};
///
/// struct Min<T> {
///     _t: std::marker::PhantomData<T>,
/// }
///
/// impl<T: Ord> Magma for Min<T> {
///     type Set = T;
///     fn op(x: Self::Set, y: Self::Set) -> Self::Set {
///         x.min(y)
///     }
/// }
/// impl<T: Ord> Commutative for Min<T> {}
///
/// assert_eq!(
///     Min::<i32>::op(3, 4),
///     Min::<i32>::op(4, 3),
/// );
/// ```
pub trait Commutative: Magma {}

/// 二項演算 $\\circ: M \\times M \\to M$ が、一部の要素について逆元を持つことを示す。
pub trait PartialRecip: Magma {
    fn partial_recip(x: Self::Set) -> Option<Self::Set>;
}

/// 二項演算 $\\circ: M \\times M \\to M$ が、常に逆元を持つことを示す。
/// $$ x \\in M \\implies {}^\\exists a \\in M: x \\circ a = a \\circ x = e. $$
/// この $a$ を $x^{-1}$ と書く。
pub trait Recip: PartialRecip {
    fn recip(x: Self::Set) -> Self::Set {
        Self::partial_recip(x).unwrap()
    }
}

/// 乗法 $\\ast: M \\times M \\to M$ は、加法 $\\circ: M \\times M \\to M$ がついて分配法則を満たすことを示す。
/// $$ \\begin{aligned}
/// x, y, z \\in R &\\implies x \\ast (y \\circ z) = (x \\ast y) \\circ (x \\ast z), \\\\
/// x, y, z \\in R &\\implies (y \\circ z) \\ast x = (y \\ast x) \\circ (z \\ast x).
/// \\end{aligned} $$
/// 加法は型引数 `A` として指定される。
pub trait Distributive<A: Magma> {}

/// 半群。マグマ $(M, \\circ)$ であり、結合法則を満たす。
pub trait Semigroup: Magma {}
impl<G: Magma> Semigroup for G {}

/// モノイド。半群 $(M, \\circ)$ であり、単位元 $e \\in M$ を持つ。
pub trait Monoid: Identity + Semigroup {}
impl<G: Identity + Semigroup> Monoid for G {}

/// 可換モノイド。モノイド $(M, \\circ, e)$ であり、交換法則を満たす。
pub trait CommutativeMonoid: Commutative + Monoid {}
impl<G: Commutative + Monoid> CommutativeMonoid for G {}

/// 群。モノイド $(M, \\circ, e)$ であり、逆元を持つ。
pub trait Group: Monoid + Recip {}
impl<G: Monoid + Recip> Group for G {}

/// 可換群。群 $(M, \\circ, e)$ であり、交換法則を満たす。
pub trait CommutativeGroup: Commutative + Monoid + Recip {}
impl<G: Commutative + Monoid + Recip> CommutativeGroup for G {}

/// 環。集合 $R$ と二つの二項演算 $\\circ$, $\\ast$ の組 $(R, \\circ, \\ast)$ であり、次の性質を満たす。
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
    fn add(x: Self::Set, y: Self::Set) -> Self::Set {
        Self::Additive::op(x, y)
    }
    /// 加法 $\\circ$ の単位元 $0$ を返す。
    fn zero() -> Self::Set {
        Self::Additive::id()
    }
    /// 加法 $\\circ$ に関する $x$ の逆元 $-x$ を返す。
    fn neg(x: Self::Set) -> Self::Set {
        Self::Additive::recip(x)
    }
    /// 積 $x \\ast y$ を返す。
    fn mul(x: Self::Set, y: Self::Set) -> Self::Set {
        Self::Multiplicative::op(x, y)
    }
    /// 乗法 $\\ast$ の単位元 $1$ を返す。
    fn one() -> Self::Set {
        Self::Multiplicative::id()
    }
}

/// 可換環。環 $(R, \\circ, \\ast, 0, 1)$ であり、$(R, \\ast, 1)$ は可換モノイドをなす。
pub trait CommutativeRing: Ring
where
    Self::Multiplicative: Commutative,
{
}

/// 体。環 $(R, \\circ, \\ast, 0, 1)$ であり、$(R \\setminus \\{0\\}, \\ast, 1)$ は群をなす。
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

/// 加法。[`std::ops::Add`](https://doc.rust-lang.org/std/ops/trait.Add.html) により定義される。
/// 単位元は [`Zero`]、逆元は [`std::ops::Neg`](https://doc.rust-lang.org/std/ops/trait.Neg.html) で定義する。
/// 結合法則を満たすときは [`AddAssoc`]、交換法則を満たすときは [`AddComm`] を実装することで示す。
///
/// [`Zero`]: trait.Zero.html
/// [`AddAssoc`]: trait.AddAssoc.html
/// [`AddComm`]: trait.AddComm.html
pub struct Additive<T> {
    _t: std::marker::PhantomData<T>,
}

use std::ops::{Add, Neg};

/// 加法の単位元 $0$ を定義する。
pub trait Zero: Add<Output = Self> + Sized {
    /// 加法の単位元 $0$ を返す。
    fn zero() -> Self;
}
/// 加法が結合法則を満たすことを示す。
/// $$ x, y, z \\in S \\implies (x + y) + z = x + (y + z). $$
pub trait AddAssoc: Add<Output = Self> + Sized {}
/// 加法が交換法則を満たすことを示す。
/// $$ x, y \\in S \\implies x + y = y + x. $$
pub trait AddComm: Add<Output = Self> + Sized {}

/// 乗法の単位元 $1$ を定義する。
pub trait One: Mul<Output = Self> + Sized {
    /// 乗法の単位元 $1$ を返す。
    fn one() -> Self;
}
/// 乗法の逆元を定義する。
pub trait MulRecip {
    /// 返り値の型。
    type Output;
    /// 乗法における $x$ の逆元 $x^{-1}$ を返す。
    fn mul_recip(self) -> Self::Output;
}
/// 乗法が結合法則を満たすことを示す。
/// $$ x, y, z \\in S \\implies (x \\times y) \\times z = x \\times (y \\times z). $$
pub trait MulAssoc: Mul<Output = Self> + Sized {}
/// 乗法が交換法則を満たすことを示す。
/// $$ x, y \\in S \\implies x \\times y = y \\times x. $$
pub trait MulComm: Mul<Output = Self> + Sized {}

impl<T> Magma for Additive<T>
where
    T: Add<Output = T> + Eq + Sized,
{
    type Set = T;
    fn op(x: Self::Set, y: Self::Set) -> Self::Set {
        x + y
    }
}
impl<T> Identity for Additive<T>
where
    T: Add<Output = T> + Eq + Sized + Zero,
{
    fn id() -> Self::Set {
        T::zero()
    }
}
impl<T> PartialRecip for Additive<T>
where
    T: Add<Output = T> + Eq + Sized + Neg<Output = T>,
{
    fn partial_recip(x: Self::Set) -> Option<Self::Set> {
        Some(-x)
    }
}
impl<T> Recip for Additive<T>
where
    T: Add<Output = T> + Eq + Sized + Neg<Output = T>,
{
    fn recip(x: Self::Set) -> Self::Set {
        -x
    }
}
impl<T> Associative for Additive<T> where
    T: Add<Output = T> + Eq + Sized + AddAssoc
{
}
impl<T> Commutative for Additive<T> where
    T: Add<Output = T> + Eq + Sized + AddComm
{
}

/// 乗法。[`std::ops::Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html) により定義される。
/// 単位元は [`One`]、逆元は [`MulRecip`] で定義する。
/// 結合法則を満たすときは [`MulAssoc`]、交換法則を満たすときは [`MulComm`] を実装することで示す。
///
/// [`One`]: trait.One.html
/// [`MulRecip`]: trait.MulRecip.html
/// [`MulAssoc`]: trait.MulAssoc.html
/// [`MulComm`]: trait.MulComm.html
pub struct Multiplicative<T> {
    _t: std::marker::PhantomData<T>,
}

use std::ops::Mul;

impl<T> Magma for Multiplicative<T>
where
    T: Mul<Output = T> + Eq + Sized,
{
    type Set = T;
    fn op(x: Self::Set, y: Self::Set) -> Self::Set {
        x * y
    }
}
impl<T> Identity for Multiplicative<T>
where
    T: Mul<Output = T> + Eq + Sized + One,
{
    fn id() -> Self::Set {
        Self::Set::one()
    }
}
impl<T> PartialRecip for Multiplicative<T>
where
    T: Mul<Output = T> + Eq + Sized + MulRecip<Output = T>,
{
    fn partial_recip(x: Self::Set) -> Option<Self::Set> {
        Some(x.mul_recip())
    }
}
impl<T> Recip for Multiplicative<T> where
    T: Mul<Output = T> + Eq + Sized + MulRecip<Output = T>
{
}
impl<T> Associative for Multiplicative<T> where
    T: Mul<Output = T> + Eq + Sized + MulAssoc
{
}
impl<T> Commutative for Multiplicative<T> where
    T: Mul<Output = T> + Eq + Sized + MulComm
{
}

macro_rules! impl_trait {
    (
        $( impl ($T:ty) for { $( $U:ty ),* } $S:tt )*
    ) => {
        $( $( impl $T for $U $S )* )*
    };
}

impl_trait! {
    impl (Zero) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {
        fn zero() -> Self { 0 }
    }
    impl (AddAssoc) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
    impl (AddComm) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
    impl (One) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {
        fn one() -> Self { 1 }
    }
    impl (MulAssoc) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
    impl (MulComm) for {i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize} {}
}

macro_rules! impl_distributive {
    ( $T:ty ) => {
        impl Distributive<Additive<$T>> for Multiplicative<$T> {}
    };
    ( $( $T:ty, )* ) => { $( impl_distributive!($T); )* };
}

impl_distributive! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}
