//! ねこちゃんライブラリ。
//!
//! [![verify](https://github.com/rsk0315/library-rs/workflows/verify/badge.svg)](https://github.com/rsk0315/library-rs/actions/workflows/verify.yml)
//! [![doctest](https://github.com/rsk0315/library-rs/workflows/doctest/badge.svg)](https://github.com/rsk0315/library-rs/actions/workflows/doc_test.yml)
//! [![unittest](https://github.com/rsk0315/library-rs/workflows/unittest/badge.svg)](https://github.com/rsk0315/library-rs/actions/workflows/unit_test.yml)
//!
//! [えびちゃん](https://twitter.com/rsk0315_h4x) の競プロライブラリですにゃ。
//!
//! 各種スニペットを個別クレートとして分けて作ったもの
//! ([`library-rs`](https://github.com/rsk0315/library-rs))
//! をモジュールの形に自動変換して作られたものがこれ (`nekolib`) になります。
//! `nekolib` は仮称で、よりよい名前が見つかったら変わると思います。
//!
//! モジュールの形に変換している理由は、主にドキュメントの見やすさのためです。
//! 元コードでクレートに分けて書いている理由は、詳しくはここでは書きません[^1]
//! が、 主には依存関係を書きやすかったためです。
//!
//! [^1]: 一度書いたところ、ここに載せるには長くなりすぎたため。
//!
//! # 🐱 Cat
//! にゃー。
//! 
//! ## Revision
//! [`359e060f7a6aaa98d4f57e91b763b1a5bba417a2`](https://github.com/rsk0315/library-rs/tree/359e060f7a6aaa98d4f57e91b763b1a5bba417a2)
//! 
//! ```text
//! +----------------+
//! |                |
//! |       o o      |
//! |      . * o     |
//! |     o S =.     |
//! |  . . + .+ + .  |
//! | . . o .. +.B   |
//! |o.  o .E .oB    |
//! |o...   ...oo+   |
//! +----------------+
//! ```
pub mod algo;
pub mod ds;
pub mod graph;
pub mod math;
pub mod seq;
pub mod traits;
pub mod utils;
