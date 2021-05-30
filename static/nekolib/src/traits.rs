//! トレイトたち。
//!
//! ここに何かを書く。
pub mod act;
pub mod action;
pub mod additive;
pub mod assoc_val;
pub mod binop;
pub mod disjoint_set;
pub mod elastic_slice;
pub mod fold;
pub mod fold_bisect;
pub mod get_mut;
pub mod max;
pub mod min;
pub mod multiplicative;
pub mod push_pop;
pub mod range_bounds;
pub mod rank_select;
pub mod set_value;
pub mod stateful_predicate;

#[doc(inline)]
pub use act::Act;
#[doc(inline)]
pub use action::MonoidAction;
#[doc(inline)]
pub use additive::{AddAssoc, AddComm, Zero};
#[doc(inline)]
pub use assoc_val::AssocVal;
#[doc(inline)]
pub use binop::{
    Associative, Commutative, CommutativeGroup, CommutativeMonoid,
    CommutativeRing, Distributive, Field, Group, Identity, Magma, Monoid,
    PartialRecip, Recip, Ring, Semigroup,
};
#[doc(inline)]
pub use disjoint_set::DisjointSet;
#[doc(inline)]
pub use elastic_slice::{
    ElasticSlice, ExpandBack, ExpandFront, ShrinkBack, ShrinkFront, SliceHash,
};
#[doc(inline)]
pub use fold::Fold;
#[doc(inline)]
pub use fold_bisect::{FoldBisect, FoldBisectRev};
#[doc(inline)]
pub use get_mut::GetMut;
#[doc(inline)]
pub use max::Max;
#[doc(inline)]
pub use min::Min;
#[doc(inline)]
pub use multiplicative::{MulAssoc, MulComm, MulRecip, One};
#[doc(inline)]
pub use push_pop::{Pop, PopBack, PopFront, Push, PushBack, PushFront};
#[doc(inline)]
pub use range_bounds::{
    EndBounded, EndExclusive, EndInclusive, EndUnbounded, StartBounded,
    StartInclusive, StartUnbounded,
};
#[doc(inline)]
pub use rank_select::RankSelect;
#[doc(inline)]
pub use set_value::SetValue;
#[doc(inline)]
pub use stateful_predicate::StatefulPred;
