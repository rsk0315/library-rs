//! アルゴリズムたち。
//!
//! ここに何かを書く。
pub mod bisect_;
pub mod exact_cover;
pub mod extremum;
pub mod extremum_float;
pub mod hilbert_mo_;
pub mod index_order;
pub mod karatsuba;
pub mod larsch;
pub mod majority_;
pub mod minmax;
pub mod mo;
pub mod ordered_hash_;
pub mod parallel_bisect;
pub mod permutation;
pub mod rle;
pub mod tortoise_hare;
pub mod window_bisect;

#[doc(inline)]
pub use bisect_::{bisect, bisect_slice};
#[doc(inline)]
pub use exact_cover::ExactCover;
#[doc(inline)]
pub use extremum::{extremum, extremum_slice};
#[doc(inline)]
pub use extremum_float::extremum_float;
#[doc(inline)]
pub use hilbert_mo_::hilbert_mo;
#[doc(inline)]
pub use index_order::{index_order_by, index_order_by_key};
#[doc(inline)]
pub use karatsuba::convolve;
#[doc(inline)]
pub use larsch::Larsch;
#[doc(inline)]
pub use majority_::majority;
#[doc(inline)]
pub use minmax::{minmax, minmax_by, minmax_by_key};
#[doc(inline)]
pub use mo::mo;
#[doc(inline)]
pub use ordered_hash_::ordered_hash;
#[doc(inline)]
pub use parallel_bisect::parallel_bisect;
#[doc(inline)]
pub use permutation::{
    next_permutation, prev_permutation, Backward, Forward, Permutations,
};
#[doc(inline)]
pub use rle::{Rle, RleBy, RleByKey};
#[doc(inline)]
pub use tortoise_hare::{cycle_mu_lambda, cycle_nth};
#[doc(inline)]
pub use window_bisect::window_bisect;
