use std::marker::PhantomData;
use std::ops::{Add, Range};

use serde::{Deserialize, Serialize};

use additive::*;
use fold::Fold;
use fold_bisect::FoldBisect;
use op_add::OpAdd;
use set_value::SetValue;

use crate::jury;
use crate::test_set::*;

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub struct Pair(u64, u64);

impl Add for Pair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl Zero for Pair {
    fn zero() -> Self {
        Self(0, 0)
    }
}
impl AddAssoc for Pair {}

pub struct Aoj0564<D> {
    _d: PhantomData<D>,
}

impl<D> Solver for Aoj0564<D>
where
    D: From<Vec<Pair>>
        + Fold<Range<usize>, Output = OpAdd<Pair>>
        + SetValue<usize, Input = Pair>
        + FoldBisect<Folded = OpAdd<Pair>>,
{
    type Jury = jury::Aoj0564;
    fn solve(mut ab: Vec<(u64, u64)>) -> u64 {
        let n = ab.len();

        ab.sort_unstable();
        let mut ab: Vec<_> = ab
            .into_iter()
            .enumerate()
            .map(|(i, (a, b))| (a, b, i))
            .collect();

        ab.sort_unstable_by(|x, y| y.1.cmp(&x.1));
        let ab = ab;

        let mut res = 0;
        let mut dp: D = vec![Pair(0, 0); n].into();
        for (a, b, i) in ab {
            let pred = |&Pair(a, k): &Pair| a <= b * k;
            dp.set_value(i, Pair(a, 1));
            let ir = dp.fold_bisect(0, pred).unwrap_or(n);
            res = res.max(dp.fold(0..ir).1);
        }

        res
    }
}
