//! トレイトたち。
//!
//! ここに何かを書く。
pub mod additive;
#[doc(inline)]
pub use additive::*;
pub mod assoc_val;
#[doc(inline)]
pub use assoc_val::*;
pub mod binop;
#[doc(inline)]
pub use binop::*;
pub mod disjoint_set;
#[doc(inline)]
pub use disjoint_set::*;
pub mod elastic_slice;
#[doc(inline)]
pub use elastic_slice::*;
pub mod fold;
#[doc(inline)]
pub use fold::*;
pub mod fold_bisect;
#[doc(inline)]
pub use fold_bisect::*;
pub mod max;
#[doc(inline)]
pub use max::*;
pub mod min;
#[doc(inline)]
pub use min::*;
pub mod multiplicative;
#[doc(inline)]
pub use multiplicative::*;
pub mod push_pop;
#[doc(inline)]
pub use push_pop::*;
pub mod range_bounds;
#[doc(inline)]
pub use range_bounds::*;
pub mod range_hash;
#[doc(inline)]
pub use range_hash::*;
pub mod set_value;
#[doc(inline)]
pub use set_value::*;
