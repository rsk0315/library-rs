//! アルゴリズムたち。
//!
//! ここに何かを書く。
pub mod bisect;
#[doc(inline)]
pub use bisect::*;
pub mod extremum;
#[doc(inline)]
pub use extremum::*;
pub mod extremum_float;
#[doc(inline)]
pub use extremum_float::*;
pub mod mo;
#[doc(inline)]
pub use mo::*;
pub mod parallel_bisect;
#[doc(inline)]
pub use parallel_bisect::*;
pub mod tortoise_hare;
#[doc(inline)]
pub use tortoise_hare::*;
pub mod window_bisect;
#[doc(inline)]
pub use window_bisect::*;
