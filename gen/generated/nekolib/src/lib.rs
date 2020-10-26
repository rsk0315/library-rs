//! ねこちゃんライブラリ。
//!
//! [![verify](https://github.com/rsk0315/library-rs/workflows/verify/badge.svg)](https://github.com/rsk0315/library-rs/actions?query=workflow%3Averify)
//! [![doctest](https://github.com/rsk0315/library-rs/workflows/doctest/badge.svg)](https://github.com/rsk0315/library-rs/actions?query=workflow%3Adoctest)
//!
//! [<span style="font-family: Lato; color: #C0C000;">rsk0315</span>](https://atcoder.jp/users/rsk0315)
//! の競プロライブラリです。
//! まだ bundler を作っていないため、競プロでの使用には適していないと思います。
//! と思ったのですが、全部貼りを厭わない人になれば大丈夫かもしれません。
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
pub mod algo;
pub mod ds;
pub mod graph;
pub mod math;
pub mod seq;
pub mod traits;
pub mod utils;
