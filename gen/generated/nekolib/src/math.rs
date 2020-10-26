//! 数学関連のアルゴリズムたち。
//!
//! 多項式や線形代数など。
pub mod modint;
#[doc(inline)]
pub use modint::*;
pub mod polynomial;
#[doc(inline)]
pub use polynomial::*;
