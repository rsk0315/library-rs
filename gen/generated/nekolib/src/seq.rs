//! 文字列アルゴリズムたち。
//!
//! 添字関連の事情で、`String` よりは `Vec<T>` として作る気がしたので、
//! 文字列アルゴリズムというよりは列アルゴリズムっぽい名前にしてみた。
pub mod seq_sentinel;
#[doc(inline)]
pub use seq_sentinel::*;
pub mod suffix_array;
#[doc(inline)]
pub use suffix_array::*;
