//! 区間作用に関するトレイトです。

use std::ops::RangeBounds;

use action::MonoidAction;
use binop::Magma;

/// 区間作用を行う。
pub trait Act<R: RangeBounds<usize>> {
    /// `r` で指定される区間に作用を行う。
    type Action: MonoidAction;
    fn act(
        &mut self,
        r: R,
        x: <<Self::Action as MonoidAction>::Operator as Magma>::Set,
    );
}
