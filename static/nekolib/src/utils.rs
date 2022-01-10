//! 便利ちゃんたち。
//!
//! ここに何かを書く。
pub mod buf_range;
pub mod op_add;
pub mod op_add_count;
pub mod op_add_on_op_add_count;
pub mod op_add_on_op_max;
pub mod op_add_on_op_min;
pub mod op_affine;
pub mod op_affine_on_op_add_count;
pub mod op_closure;
pub mod op_closure_on_op_closure;
pub mod op_max;
pub mod op_min;
pub mod op_mul;
pub mod op_roll_hash;
pub mod scanner;

#[doc(inline)]
pub use buf_range::{bounds_within, check_bounds, check_bounds_range};
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
pub use op_max::OpMax;
#[doc(inline)]
pub use op_min::OpMin;
#[doc(inline)]
pub use op_mul::OpMul;
#[doc(inline)]
pub use op_roll_hash::OpRollHash;
#[doc(inline)]
pub use scanner::Scanner;
