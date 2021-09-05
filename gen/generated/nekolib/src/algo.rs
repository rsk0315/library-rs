//! アルゴリズムたち。
//!
//! ここに何かを書く。
pub mod bisect;
pub mod exact_cover;
pub mod extremum;
pub mod extremum_float;
pub mod karatsuba;
pub mod majority_;
pub mod minmax;
pub mod mo;
pub mod parallel_bisect;
pub mod permutation;
pub mod tortoise_hare;
pub mod window_bisect;

#[doc(inline)]
pub use bisect::{bisect, bisect_slice};
#[doc(inline)]
pub use exact_cover::ExactCover;
#[doc(inline)]
pub use extremum::{extremum, extremum_slice};
#[doc(inline)]
pub use extremum_float::extremum_float;
#[doc(inline)]
pub use karatsuba::convolve;
#[doc(inline)]
pub use majority_::majority;
#[doc(inline)]
pub use minmax::{minmax, minmax_by, minmax_by_key};
#[doc(inline)]
pub use mo::mo;
#[doc(inline)]
pub use parallel_bisect::parallel_bisect;
#[doc(inline)]
pub use permutation::next_permutation;
#[doc(inline)]
pub use tortoise_hare::tortoise_hare;
#[doc(inline)]
pub use window_bisect::window_bisect;
