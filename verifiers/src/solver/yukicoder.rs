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
    yuki_3287,
}
