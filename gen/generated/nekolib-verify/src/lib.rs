//! [`nekolib`] の verify に関するもの。
//!
//! そのうちちゃんと作ります。
//!
//! [`nekolib`]: ../nekolib/index.html
//!
//! `library-rs` では verify をしているのですが、
//! いまいち運用しやすい形式を確立できていないので、早くなんとかしたいです。
//!
//! どの問題で何を verify したかとかを見やすい形式で可視化できたらいいよね。
//! たとえば、次のような形式のドキュメントを生成しやすいように作ってみる？
//!
//! # Sample (verifier)
//!
//! some algo の verify をします。
//!
//! ## Verified by
//! - ソルバへのリンク 1
//! - ソルバへのリンク 2
//!
//! # Sample (solver)
//!
//! some algo を用いて some prob を解きます。
//!
//! ## Solves
//! - 問題へのリンク
//!
//! ## Expalnations
//! 解法の概要などが必要であれば書く。

pub mod jury;
pub mod solver;
pub mod test_set;
pub mod verifier;
