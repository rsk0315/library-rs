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
    aoj_0000_wa,
    aoj_0000_re,
    aoj_0000_tle,
    aoj_0002,
    yuki_3287,
}
