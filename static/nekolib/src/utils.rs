//! 便利ちゃんたち。
//!
//! ここに何かを書く。
pub mod ascii;
pub mod bitop;
pub mod buf_range;
pub mod e_macro;
pub mod make_minmax;
pub mod op_add;
pub mod op_add_count;
pub mod op_add_on_op_add_count;
pub mod op_add_on_op_max;
pub mod op_add_on_op_min;
pub mod op_affine;
pub mod op_affine_on_op_add_count;
pub mod op_closure;
pub mod op_closure_on_op_closure;
pub mod op_gcd;
pub mod op_max;
pub mod op_min;
pub mod op_mul;
pub mod op_roll_hash;
pub mod output;
pub mod rand_gen_macro;
pub mod scanner;

#[doc(inline)]
pub use ascii::{
    charset, ASCII, ASCII_ALPHABETIC, ASCII_ALPHANUMERIC, ASCII_CONTROL,
    ASCII_DIGIT, ASCII_GRAPHIC, ASCII_HEXDIGIT, ASCII_LOWERCASE,
    ASCII_PUNCTUATION, ASCII_UPPERCASE, ASCII_WHITESPACE,
};
#[doc(inline)]
pub use bitop::{
    Pdep, PdepPextMaskU128, PdepPextMaskU16, PdepPextMaskU32, PdepPextMaskU64,
    PdepPextMaskU8, Pext,
};
#[doc(inline)]
pub use buf_range::{bounds_within, check_bounds, check_bounds_range};
#[doc(inline)]
pub use make_minmax::{MakeMax, MakeMin};
#[doc(inline)]
pub use op_add::OpAdd;
#[doc(inline)]
pub use op_add_count::OpAddCount;
#[doc(inline)]
pub use op_add_on_op_add_count::OpAddOnOpAddCount;
#[doc(inline)]
pub use op_add_on_op_max::OpAddOnOpMax;
#[doc(inline)]
pub use op_add_on_op_min::OpAddOnOpMin;
#[doc(inline)]
pub use op_affine::OpAffine;
#[doc(inline)]
pub use op_affine_on_op_add_count::OpAffineOnOpAddCount;
#[doc(inline)]
pub use op_closure::OpClosure;
#[doc(inline)]
pub use op_closure_on_op_closure::OpClosureOnOpClosure;
#[doc(inline)]
pub use op_gcd::OpGcd;
#[doc(inline)]
pub use op_max::OpMax;
#[doc(inline)]
pub use op_min::OpMin;
#[doc(inline)]
pub use op_mul::OpMul;
#[doc(inline)]
pub use op_roll_hash::OpRollHash;
#[doc(inline)]
pub use output::{PerLine, SpaceSep, StrSep};
#[doc(inline)]
pub use rand_gen_macro::{RandomGenerator, VecMarker};
#[doc(inline)]
pub use scanner::Scanner;

// pub mod scan_macro;
// #[doc(inline)]
// pub use scan_macro::{AutoSource, OnceSource, Readable};
