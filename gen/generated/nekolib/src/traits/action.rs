//! 作用モノイド。

use super::binop;

use binop::{Magma, Monoid};

/// 作用モノイド。
pub trait MonoidAction {
    /// 作用を行う型。
    type Operator: Monoid;
    /// 作用される型。
    type Operand: Monoid;
    /// 作用を行う。
    fn act(
        x: &mut <Self::Operand as Magma>::Set,
        op: <Self::Operator as Magma>::Set,
    );
}
