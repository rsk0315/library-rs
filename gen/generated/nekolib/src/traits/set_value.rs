//! 値の代入に関するトレイトです。

/// 値の代入ができることを示す。
///
/// 典型的には、`I` が `usize` であれば特定の要素に対する代入を指し、
/// `Range<usize>` であれば区間に対する代入を指す。
pub trait SetValue<I> {
    /// 代入される型。
    type Input;
    /// `i` で指定される要素に `x` を代入する。
    fn set_value(&mut self, i: I, x: Self::Input);
}
