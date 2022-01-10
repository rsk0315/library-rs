use super::op_add;
use super::op_max;
use super::super::traits::action;
use super::super::traits::additive;
use super::super::traits::binop;
use super::super::traits::min;
use std::fmt::Debug;
use std::ops::Add;

use action::MonoidAction;
use additive::{AddAssoc, Zero};
use binop::Magma;
use min::Min;
use op_add::OpAdd;
use op_max::OpMax;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpAddOnOpMax<T> {
    op_add: OpAdd<T>,
    op_max: OpMax<T>,
}

impl<T: Ord + Eq + Add<Output = T> + AddAssoc + Zero + Min + Sized> MonoidAction
    for OpAddOnOpMax<T>
{
    type Operand = OpMax<T>;
    type Operator = OpAdd<T>;
    fn operand(&self) -> &Self::Operand { &self.op_max }
    fn operator(&self) -> &Self::Operator { &self.op_add }
    fn act(&self, x: T, op: T) -> T { self.op_add.op(x, op) }
}
