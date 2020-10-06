pub trait Min: Ord {
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
