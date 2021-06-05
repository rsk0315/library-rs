//! 数学関連のアルゴリズムたち。
//!
//! 多項式や線形代数など。
pub mod modint;
// pub mod polynomial;
pub mod count_prime;
pub mod divisors_;
pub mod factors_;
pub mod gcd_;
pub mod harmonic_sum;
pub mod lcm_;
pub mod linear_floor_sum_;
pub mod linear_sieve;
pub mod totient_phi_;

#[doc(inline)]
pub use count_prime::prime_pi;
#[doc(inline)]
pub use divisors_::divisors;
#[doc(inline)]
pub use factors_::{factors, factors_dup};
#[doc(inline)]
pub use gcd_::gcd;
#[doc(inline)]
pub use harmonic_sum::HarmonicSum;
#[doc(inline)]
pub use lcm_::{lcm, overflowing_lcm};
#[doc(inline)]
pub use linear_floor_sum_::linear_floor_sum;
#[doc(inline)]
pub use linear_sieve::LinearSieve;
#[doc(inline)]
pub use modint::ModInt;
#[doc(inline)]
pub use totient_phi_::totient_phi;
