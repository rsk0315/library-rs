macro_rules! uses {
    ( $( $i:ident, )* ) => {
        $(
            pub mod $i;
            #[doc(inline)]
            pub use $i::*;
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
    yuki_3287,
}
