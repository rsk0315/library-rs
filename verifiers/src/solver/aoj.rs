macro_rules! uses {
    (
        $($c:ident,)*
    ) => {
        $(
            pub mod $c;
            #[doc(inline)]
            pub use $c::*;
        )*
    }
}

uses! {
    aoj_0000,
    aoj_dsl_2_b,
}
