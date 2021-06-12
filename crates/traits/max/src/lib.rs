//! 最大元に関するトレイトです。

/// 最大元を持つ。
///
/// # Examples
/// ```
/// use nekolib::traits::Max;
///
/// let max: i8 = Max::max();
/// for x in std::i8::MIN..=std::i8::MAX {
///     assert!(x <= max);
/// }
/// ```
pub trait Max: Ord {
    /// 最大元を返す。
    fn max() -> Self;
}

macro_rules! impl_max {
    (
        $( $t:ident ),*
    ) => {
        $(
            impl Max for $t {
                fn max() -> Self {
                    std::$t::MAX
                }
            }
        )*
    }
}

impl_max! {
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, char
}
