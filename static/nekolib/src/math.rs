//! 数学関連のアルゴリズムたち。
//!
//! 基本的な数学関数。
//! 直線の集合を管理するクラスなど、抽象化しにくいものは [`ds`] ではなくこちらに含めた。
//! 計算機科学自体 mathematics では？とも思うが...
//!
//! [`ds`]: ../ds/index.html
pub mod bit_binom_;
pub mod carmichael_lambda;
pub mod common_quot;
pub mod compact_sieve;
pub mod const_div;
pub mod continued_fraction_;
pub mod modint;
pub mod polynomial;
// pub mod count_prime;
pub mod convolution;
pub mod digit_sum;
pub mod digits;
pub mod divisors;
pub mod dlog;
pub mod equiv_mod;
pub mod euler_phi;
pub mod factors;
pub mod factors_dup;
pub mod frac_approx;
pub mod fraction_bisect;
pub mod garner;
pub mod gcd;
pub mod gcd_recip;
pub mod harmonic_floor_sum;
pub mod interpolation;
pub mod is_close_float;
pub mod lcm;
pub mod linear_floor_sum;
pub mod linear_sieve;
pub mod miller_rabin;
pub mod mod_ackermann;
pub mod mod_factorial_binom;
pub mod mod_ord;
pub mod mod_pow;
pub mod mod_recip_table_;
pub mod mod_tetration;
pub mod prime_pi_;
pub mod segmented_factor_sieve;
pub mod sieve_n2_plus_1;
pub mod sieve_n2_plus_n_plus_1;
pub mod slope_function;
pub mod sqrt;
pub mod sqrt_fraction_;
pub mod stern_brocot_;
pub mod two_sat;

#[doc(inline)]
pub use bit_binom_::bit_binom;
#[doc(inline)]
pub use carmichael_lambda::CarmichaelLambda;
#[doc(inline)]
pub use common_quot::CommonQuot;
#[doc(inline)]
pub use compact_sieve::CompactSieve;
#[doc(inline)]
pub use const_div::{ConstDiv, ConstDiv2};
#[doc(inline)]
pub use continued_fraction_::continued_fraction;
#[doc(inline)]
pub use convolution::{
    butterfly, butterfly_inv, convolve, convolve_u128, convolve_u128_mod,
    convolve_u32_mod, convolve_u64, convolve_u64_mod, convolve_wrapping_u128,
    convolve_wrapping_u64, ButterflyCache, NttFriendly,
};
#[doc(inline)]
pub use digits::Digits;
#[doc(inline)]
pub use frac_approx::{ApproxFrac, FracApprox, FracApproxIter};
#[doc(inline)]
pub use fraction_bisect::FractionBisect;
#[doc(inline)]
pub use garner::{CrtMod, CrtWrapping};
#[doc(inline)]
pub use is_close_float::IsCloseFloat;
#[doc(inline)]
pub use miller_rabin::MillerRabin;
#[doc(inline)]
pub use polynomial::Polynomial;
#[doc(inline)]
pub use segmented_factor_sieve::SegmentedFactorSieve;
// #[doc(inline)]
// pub use count_prime::prime_pi;
#[doc(inline)]
pub use digit_sum::DigitSum;
#[doc(inline)]
pub use divisors::Divisors;
#[doc(inline)]
pub use dlog::DLog;
#[doc(inline)]
pub use equiv_mod::{EquivMod, EquivModIter};
#[doc(inline)]
pub use euler_phi::EulerPhi;
#[doc(inline)]
pub use factors::Factors;
#[doc(inline)]
pub use factors_dup::FactorsDup;
#[doc(inline)]
pub use gcd::Gcd;
#[doc(inline)]
pub use gcd_recip::GcdRecip;
#[doc(inline)]
pub use harmonic_floor_sum::HarmonicFloorSum;
#[doc(inline)]
pub use interpolation::Interpolation;
#[doc(inline)]
pub use lcm::Lcm;
#[doc(inline)]
pub use linear_floor_sum::LinearFloorSum;
#[doc(inline)]
pub use linear_sieve::LinearSieve;
#[doc(inline)]
pub use mod_ackermann::ModAckermann;
#[doc(inline)]
pub use mod_factorial_binom::ModFactorialBinom;
#[doc(inline)]
pub use mod_ord::ModOrd;
#[doc(inline)]
pub use mod_pow::ModPow;
#[doc(inline)]
pub use mod_recip_table_::mod_recip_table_prime;
#[doc(inline)]
pub use mod_tetration::ModTetration;
#[doc(inline)]
pub use modint::{
    DefaultId, DynamicModInt, Mod1000000007, Mod998244353, ModInt1000000007,
    ModInt998244353, ModIntBase, Modulus, StaticModInt,
};
#[doc(inline)]
pub use prime_pi_::prime_pi;
#[doc(inline)]
pub use sieve_n2_plus_1::SieveN2Plus1;
#[doc(inline)]
pub use sieve_n2_plus_n_plus_1::SieveN2PlusNPlus1;
#[doc(inline)]
pub use slope_function::SlopeFunction;
#[doc(inline)]
pub use sqrt::Sqrt;
#[doc(inline)]
pub use sqrt_fraction_::{sqrt_fraction, sqrt_fraction_fn};
#[doc(inline)]
pub use stern_brocot_::stern_brocot;
#[doc(inline)]
pub use two_sat::TwoSat;
