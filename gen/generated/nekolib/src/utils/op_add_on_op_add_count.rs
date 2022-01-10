use super::op_add;
use super::op_add_count;
use super::super::traits::action;
use super::super::traits::additive;
use std::fmt::Debug;
use std::ops::{Add, Mul};

use action::MonoidAction;
use additive::{AddAssoc, Zero};
use op_add::OpAdd;
use op_add_count::OpAddCount;

#[derive(Clone, Copy, Debug, Default)]
pub struct OpAddOnOpAddCount<T> {
    op_add: OpAdd<T>,
    op_add_count: OpAddCount<T>,
}

impl<T> MonoidAction for OpAddOnOpAddCount<T>
where
    T: Ord
        + Eq
        + Clone
        + Add<Output = T>
        + AddAssoc
        + Mul<Output = T>
        + Zero
        + Sized,
{
    type Operand = OpAddCount<T>;
    type Operator = OpAdd<T>;
    fn operand(&self) -> &Self::Operand { &self.op_add_count }
    fn operator(&self) -> &Self::Operator { &self.op_add }
    fn act(&self, (xv, xc): (T, T), op: T) -> (T, T) {
        let xv = xv + op * xc.clone();
        (xv, xc)
    }
}
