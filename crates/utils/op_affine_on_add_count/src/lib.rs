use std::fmt::Debug;
use std::ops::{Add, Mul};

use action::MonoidAction;
use additive::{AddAssoc, Zero};
use multiplicative::One;
use op_add_count::OpAddCount;
use op_affine::OpAffine;

#[derive(Clone, Copy, Debug, Default)]
struct OpAffineOnOpAddCount<T> {
    op_affine: OpAffine<T>,
    op_add_count: OpAddCount<T>,
}

impl<T> MonoidAction for OpAffineOnOpAddCount<T>
where
    T: Ord
        + Eq
        + Clone
        + Add<Output = T>
        + AddAssoc
        + Mul<Output = T>
        + Zero
        + One
        + Sized,
{
    type Operand = OpAddCount<T>;
    type Operator = OpAffine<T>;
    fn operand(&self) -> &Self::Operand { &self.op_add_count }
    fn operator(&self) -> &Self::Operator { &self.op_affine }
    fn act(&self, (xv, xc): (T, T), (y1, y0): (T, T)) -> (T, T) {
        // Sum(ax+b) = a Sum(x) + b Sum(1)
        let xv = y1 * xv + y0 * xc.clone();
        (xv, xc)
    }
}
