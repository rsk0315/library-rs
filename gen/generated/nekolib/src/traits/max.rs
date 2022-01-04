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

impl Max for bool {
    fn max() -> Self { true }
}

impl Max for () {
    fn max() -> Self { () }
}

impl<A: Max> Max for (A,) {
    fn max() -> Self { (Max::max(),) }
}

macro_rules! impl_tuple {
    (
        ( $( $t:ident ),+ )
    ) => {
        impl < $( $t: Max ),+ > Max for ( $( $t ),+ ) {
            fn max() -> Self { ( $( <$t as Max>::max() ),+ ) }
        }
    };
    ( $( ( $( $t:ident ),+ ), )+ ) => {
        $( impl_tuple! { ( $( $t ),+ ) } )+
    }
}

impl_tuple! {
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
}

#[test]
fn test() {
    assert_eq!(<() as Max>::max(), ());
    assert_eq!(<u8 as Max>::max(), 255);
    assert_eq!(<char as Max>::max(), '\u{10ffff}');

    let i32_max: i32 = Max::max();
    let char_max: char = Max::max();
    let u128_max: u128 = Max::max();
    assert_eq!(
        <(i32, char, u128) as Max>::max(),
        (i32_max, char_max, u128_max)
    );
}
