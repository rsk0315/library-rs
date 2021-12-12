//! ねこちゃんライブラリ。
//!
//! [![verify](https://github.com/rsk0315/library-rs/actions/workflows/verify.yml/badge.svg)](https://github.com/rsk0315/library-rs/actions/workflows/verify.yml)
//! [![doctest](https://github.com/rsk0315/library-rs/actions/workflows/doc_test.yml/badge.svg)](https://github.com/rsk0315/library-rs/actions/workflows/doc_test.yml)
//! [![unittest](https://github.com/rsk0315/library-rs/actions/workflows/unit_test.yml/badge.svg)](https://github.com/rsk0315/library-rs/actions/workflows/unit_test.yml)
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
//! [`d6ce510110d6776a7994e6f71a73c622b3039b07`](https://github.com/rsk0315/library-rs/tree/d6ce510110d6776a7994e6f71a73c622b3039b07)
//! 
//! ```text
//! +----------------+
//! |       ++.... . |
//! |      .  . o =  |
//! |        . o B   |
//! |       S o + o .|
//! |      . oEo . o.|
//! |         o=o + =|
//! |         o o+ B |
//! |          ....  |
//! +----------------+
//! ```
pub mod algo;
pub mod ds;
pub mod graph;
pub mod math;
pub mod seq;
pub mod traits;
pub mod utils;
