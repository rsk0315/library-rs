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
    aoj_0270,
    aoj_0425,
    aoj_0564,
    aoj_0575,
    aoj_1180,
    aoj_alds1_14_b,
    aoj_alds1_14_b_kmp,
    aoj_alds1_14_b_z,
    aoj_alds1_14_d,
    aoj_dsl_1_a,
    aoj_dsl_1_b,
    aoj_dsl_2_b,
    aoj_grl_1_a,
    aoj_grl_3_c,
    yuki_3287,
}
