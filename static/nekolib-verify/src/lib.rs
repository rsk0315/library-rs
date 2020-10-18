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
//! ---
//!
//! # Sample (verifier)
//!
//! `some_algo` の verify をします。
//!
//! ## Verified by
//! - ソルバへのリンク 1 (passing/failing)
//! - ソルバへのリンク 2 (passing/failing)
//! - ソルバへのリンク 3 (passing/failing)
//!
//! ---
//!
//! # Sample (solver for algo)
//!
//! `some_algo` を用いて問題 A を解きます。
//!
//! ## Solves
//! - 問題 A へのリンク
//!
//! ## Explanations
//! 解法の概要などが必要であれば書く。
//!
//! ---
//!
//! # Sample (solver for ds)
//!
//! トレイト `T` を実装した `some_ds` を用いて問題 B を解きます。
//!
//! ## Solves
//! - 問題 B へのリンク
//!
//! ## Explanations
//! 解法の概要などが必要であれば書く。

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
