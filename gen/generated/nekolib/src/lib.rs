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
//! [`7257113556d3b79335a436c8349ee5e22371e7dd`](https://github.com/rsk0315/library-rs/tree/7257113556d3b79335a436c8349ee5e22371e7dd)
//! 
//! ```text
//! +----------------+
//! |          oo==+o|
//! |         + B..o=|
//! |        . O *  *|
//! |     . S = = o+.|
//! |      o o o . .E|
//! |         . .    |
//! |                |
//! |                |
//! +----------------+
//! ```
pub mod algo;
pub mod ds;
pub mod graph;
pub mod math;
pub mod seq;
pub mod traits;
pub mod utils;
