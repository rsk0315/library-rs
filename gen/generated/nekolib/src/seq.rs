//! 文字列アルゴリズムたち。
//!
//! 添字関連の事情で、`String` よりは `Vec<T>` として作る気がしたので、
//! 文字列アルゴリズムというよりは列アルゴリズムっぽい名前にしてみました。
pub mod kmp;
#[doc(inline)]
pub use kmp::*;
pub mod suffix_array;
#[doc(inline)]
pub use suffix_array::*;
