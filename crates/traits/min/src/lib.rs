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

impl Min for bool {
    fn min() -> Self { false }
}

impl Min for char {
    fn min() -> Self { '\0' }
}

impl Min for () {
    fn min() -> Self { () }
}

impl<A: Min> Min for (A,) {
    fn min() -> Self { (Min::min(),) }
}

macro_rules! impl_tuple {
    (
        ( $( $t:ident ),+ )
    ) => {
        impl < $( $t: Min ),+ > Min for ( $( $t ),+ ) {
            fn min() -> Self { ( $( <$t as Min>::min() ),+ ) }
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
    assert_eq!(<() as Min>::min(), ());
    assert_eq!(<u8 as Min>::min(), 0);
    assert_eq!(<char as Min>::min(), '\0');

    let i32_min: i32 = Min::min();
    let char_min: char = Min::min();
    let u128_min: u128 = Min::min();
    assert_eq!(
        <(i32, char, u128) as Min>::min(),
        (i32_min, char_min, u128_min)
    );
}
