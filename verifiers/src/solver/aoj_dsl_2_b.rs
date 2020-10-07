use std::marker::PhantomData;
use std::ops::{Index, Range};

use fold::Fold;
use op_add::OpAdd;
use set_value::SetValue;

use crate::jury;
use crate::test_set::*;

use jury::aoj_dsl_2_b::Query;

pub struct AojDsl2B<D> {
    _d: PhantomData<D>,
}

impl<D> Solver for AojDsl2B<D>
where
    D: From<Vec<u64>>
        + SetValue<usize, Input = u64>
        + Index<usize, Output = u64>
        + Fold<Range<usize>, Output = OpAdd<u64>>,
{
    type Jury = jury::AojDsl2B;
    fn solve((n, qs): (usize, Vec<Query>)) -> Vec<u64> {
        let mut rq: D = vec![0u64; n].into();
        qs.into_iter()
            .filter_map(|q| match q {
                Query::Add(i, x) => {
                    rq.set_value(i, rq[i] + x);
                    None
                }
                Query::GetSum(s, t) => Some(rq.fold(s..t)),
            })
            .collect()
    }
}
