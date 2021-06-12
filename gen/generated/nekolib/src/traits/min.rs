//! 最小元に関するトレイトです。

/// 最小元を持つ。
///
/// # Examples
/// ```
/// use nekolib::traits::Min;
///
/// let min: i8 = Min::min();
/// for x in std::i8::MIN..=std::i8::MAX {
///     assert!(x >= min);
/// }
/// ```
pub trait Min: Ord {
    /// 最小元を返す。
    fn min() -> Self;
}

macro_rules! impl_min {
    (
        $( $t:ident ),*
    ) => {
        $(
            impl Min for $t {
                fn min() -> Self {
                    std::$t::MIN
                }
            }
        )*
    }
}

impl_min! {
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
}
