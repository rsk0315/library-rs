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
    aoj_0002,
    aoj_0270,
    aoj_0564,
    aoj_1180,
    aoj_dsl_2_b,
}
