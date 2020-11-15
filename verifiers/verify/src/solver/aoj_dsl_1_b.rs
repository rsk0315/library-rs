use std::marker::PhantomData;

use crate::jury;
use crate::test_set::Solver;

use op_add::OpAdd;
use potential_function::PotentialFunction;

use jury::aoj_dsl_1_b::Query;

pub struct AojDsl1B<D> {
    _d: PhantomData<D>,
}

impl<D> Solver for AojDsl1B<D>
where
    D: PotentialFunction<Item = OpAdd<i32>>,
{
    type Jury = jury::AojDsl1B;
    fn solve((n, qs): (usize, Vec<Query>)) -> Vec<Option<i32>> {
        let mut ds = D::new(n);
        qs.into_iter()
            .filter_map(|q| match q {
                Query::Relate(u, v, w) => {
                    ds.relate(v, u, w);
                    None
                }
                Query::Diff(u, v) => Some(ds.diff(v, u)),
            })
            .collect()
    }
}
