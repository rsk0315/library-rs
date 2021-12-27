use std::fmt::Debug;
use std::ops::Add;

use action::MonoidAction;
use additive::{AddAssoc, Zero};
use binop::Magma;
use max::Max;
use op_add::OpAdd;
use op_min::OpMin;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpAddOnOpMin<T> {
    op_add: OpAdd<T>,
    op_min: OpMin<T>,
}

impl<T: Ord + Eq + Add<Output = T> + AddAssoc + Zero + Max + Sized> MonoidAction
    for OpAddOnOpMin<T>
{
    type Operand = OpMin<T>;
    type Operator = OpAdd<T>;
    fn operand(&self) -> &Self::Operand { &self.op_min }
    fn operator(&self) -> &Self::Operator { &self.op_add }
    fn act(&self, x: T, op: T) -> T { self.op_add.op(x, op) }
}
