//! 便利ちゃんたち。
//!
//! ここに何かを書く。
pub mod buf_range;
pub mod op_add;
pub mod op_max;
pub mod op_min;
pub mod op_mul;
pub mod op_roll_hash;
pub mod scanner;

#[doc(inline)]
pub use buf_range::bounds_within;
#[doc(inline)]
pub use op_add::OpAdd;
#[doc(inline)]
pub use op_max::OpMax;
#[doc(inline)]
pub use op_min::OpMin;
#[doc(inline)]
pub use op_mul::OpMul;
#[doc(inline)]
pub use op_roll_hash::OpRollHash;
#[doc(inline)]
pub use scanner::Scanner;
