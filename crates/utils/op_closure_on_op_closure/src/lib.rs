use std::fmt::Debug;

use action::MonoidAction;
use op_closure::OpClosure;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpClosureOnOpClosure<T, OpT, IdT, U, OpU, IdU, Act>
where
    OpT: Fn(T, T) -> T,
    IdT: Fn() -> T,
    OpU: Fn(U, U) -> U,
    IdU: Fn() -> U,
    Act: Fn(T, U) -> T,
{
    operator: OpClosure<U, OpU, IdU>,
    operand: OpClosure<T, OpT, IdT>,
    act: Act,
}

impl<T, OpT, IdT, U, OpU, IdU, Act> MonoidAction
    for OpClosureOnOpClosure<T, OpT, IdT, U, OpU, IdU, Act>
where
    T: Eq + Sized,
    OpT: Fn(T, T) -> T,
    IdT: Fn() -> T,
    U: Eq + Sized,
    OpU: Fn(U, U) -> U,
    IdU: Fn() -> U,
    Act: Fn(T, U) -> T,
{
    type Operand = OpClosure<T, OpT, IdT>;
    type Operator = OpClosure<U, OpU, IdU>;
    fn operand(&self) -> &Self::Operand { &self.operand }
    fn operator(&self) -> &Self::Operator { &self.operator }
    fn act(&self, x: T, op: U) -> T { (self.act)(x, op) }
}

impl<T, OpT, IdT, U, OpU, IdU, Act>
    OpClosureOnOpClosure<T, OpT, IdT, U, OpU, IdU, Act>
where
    T: Eq + Sized,
    OpT: Fn(T, T) -> T,
    IdT: Fn() -> T,
    U: Eq + Sized,
    OpU: Fn(U, U) -> U,
    IdU: Fn() -> U,
    Act: Fn(T, U) -> T,
{
    pub fn new(
        operand: OpClosure<T, OpT, IdT>,
        operator: OpClosure<U, OpU, IdU>,
        act: Act,
    ) -> Self {
        Self { operand, operator, act }
    }
}
