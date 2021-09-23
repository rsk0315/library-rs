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
//! [`d5c218500b8f5128cc01ab47e06f96d62c3f175f`](https://github.com/rsk0315/library-rs/tree/d5c218500b8f5128cc01ab47e06f96d62c3f175f)
//! 
//! ```text
//! +----------------+
//! |..+..+=++ .     |
//! |...+ .=..+ .    |
//! | o. .. o. .     |
//! |... +  S        |
//! |. .B o .   E    |
//! | .+ o   o .     |
//! |     o . .      |
//! |      o         |
//! +----------------+
//! ```
pub mod algo;
pub mod ds;
pub mod graph;
pub mod math;
pub mod seq;
pub mod traits;
pub mod utils;
