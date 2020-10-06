pub trait Max: Ord {
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
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
}
