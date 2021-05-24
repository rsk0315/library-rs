//! 数学関連のアルゴリズムたち。
//!
//! 多項式や線形代数など。
pub mod modint;
// pub mod polynomial;
pub mod count_prime;
pub mod harmonic_sum;
pub mod linear_floor_sum;
pub mod linear_sieve;

#[doc(inline)]
pub use count_prime::prime_pi;
#[doc(inline)]
pub use harmonic_sum::HarmonicSum;
#[doc(inline)]
pub use linear_floor_sum::linear_floor_sum;
#[doc(inline)]
pub use linear_sieve::LinearSieve;
#[doc(inline)]
pub use modint::ModInt;
