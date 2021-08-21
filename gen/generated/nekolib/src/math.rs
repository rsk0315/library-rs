//! 数学関連のアルゴリズムたち。
//!
//! 基本的な数学関数。
//! 直線の集合を管理するクラスなど、抽象化しにくいものは [`ds`] ではなくこちらに含めた。
//! 計算機科学自体 mathematics では？とも思うが...
//!
//! [`ds`]: ../ds/index.html
pub mod modint;
// pub mod polynomial;
pub mod carmichael_lambda;
pub mod compact_sieve;
pub mod const_div;
pub mod count_prime;
pub mod crt;
pub mod divisors;
pub mod dlog;
pub mod euler_phi;
pub mod factors;
pub mod gcd;
pub mod gcd_recip;
pub mod harmonic_floor_sum;
pub mod incremental_line_set;
pub mod lcm;
pub mod linear_floor_sum;
pub mod linear_sieve;
pub mod mod_ackermann;
pub mod mod_pow;
pub mod mod_tetration;
pub mod ord;
pub mod sieve_square_plus_one;
pub mod slope_function;
pub mod two_sat;

#[doc(inline)]
pub use carmichael_lambda::CarmichaelLambda;
#[doc(inline)]
pub use compact_sieve::CompactSieve;
#[doc(inline)]
pub use const_div::{ConstDiv, ConstDiv2};
#[doc(inline)]
pub use count_prime::prime_pi;
#[doc(inline)]
pub use crt::Crt;
#[doc(inline)]
pub use divisors::Divisors;
#[doc(inline)]
pub use dlog::DLog;
#[doc(inline)]
pub use euler_phi::EulerPhi;
#[doc(inline)]
pub use factors::Factors;
#[doc(inline)]
pub use gcd::Gcd;
#[doc(inline)]
pub use gcd_recip::GcdRecip;
#[doc(inline)]
pub use harmonic_floor_sum::HarmonicFloorSum;
#[doc(inline)]
pub use incremental_line_set::IncrementalLineSet;
#[doc(inline)]
pub use lcm::Lcm;
#[doc(inline)]
pub use linear_floor_sum::LinearFloorSum;
#[doc(inline)]
pub use linear_sieve::LinearSieve;
#[doc(inline)]
pub use mod_ackermann::ModAckermann;
#[doc(inline)]
pub use mod_ord::ModOrd;
#[doc(inline)]
pub use mod_pow::ModPow;
#[doc(inline)]
pub use mod_tetration::ModTetration;
#[doc(inline)]
pub use modint::ModInt;
#[doc(inline)]
pub use sieve_square_plus_one::SieveSquarePlusOne;
#[doc(inline)]
pub use slope_function::SlopeFunction;
#[doc(inline)]
pub use two_sat::TwoSat;
