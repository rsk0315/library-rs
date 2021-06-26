//! 数学関連のアルゴリズムたち。
//!
//! 基本的な数学関数。
//! 直線の集合を管理するクラスなど、抽象化しにくいものは [`ds`] ではなくこちらに含めた。
//! 計算機科学自体 mathematics では？とも思うが...
//!
//! [`ds`]: ../ds/index.html
pub mod modint;
// pub mod polynomial;
pub mod const_div;
pub mod count_prime;
pub mod divisors_;
pub mod dlog_;
pub mod factors_;
pub mod gcd_;
pub mod gcd_recip_;
pub mod harmonic_sum;
pub mod incremental_line_set;
pub mod lcm_;
pub mod linear_floor_sum_;
pub mod linear_sieve;
pub mod mod_pow_;
pub mod ord_;
pub mod slope_function;
pub mod totient_phi_;
pub mod two_sat;

#[doc(inline)]
pub use const_div::{ConstDiv, ConstDiv2};
#[doc(inline)]
pub use count_prime::prime_pi;
#[doc(inline)]
pub use divisors_::divisors;
#[doc(inline)]
pub use dlog_::dlog;
#[doc(inline)]
pub use factors_::{factors, factors_dup};
#[doc(inline)]
pub use gcd_::gcd;
#[doc(inline)]
pub use gcd_recip_::gcd_recip;
#[doc(inline)]
pub use harmonic_sum::HarmonicSum;
#[doc(inline)]
pub use incremental_line_set::IncrementalLineSet;
#[doc(inline)]
pub use lcm_::{lcm, overflowing_lcm};
#[doc(inline)]
pub use linear_floor_sum_::linear_floor_sum;
#[doc(inline)]
pub use linear_sieve::LinearSieve;
#[doc(inline)]
pub use mod_pow_::{mod_pow, mod_pow_with_cd};
#[doc(inline)]
pub use modint::ModInt;
#[doc(inline)]
pub use ord_::ord;
#[doc(inline)]
pub use slope_function::SlopeFunction;
#[doc(inline)]
pub use totient_phi_::totient_phi;
#[doc(inline)]
pub use two_sat::TwoSat;
