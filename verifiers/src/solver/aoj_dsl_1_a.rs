use std::marker::PhantomData;

use disjoint_set::DisjointSet;

use crate::jury;
use crate::test_set::Solver;

use jury::aoj_dsl_1_a::Query;

pub struct AojDsl1A<D> {
    _d: PhantomData<D>,
}

impl<D> Solver for AojDsl1A<D>
where
    D: DisjointSet,
{
    type Jury = jury::AojDsl1A;
    fn solve((n, qs): (usize, Vec<Query>)) -> Vec<bool> {
        let mut ds = D::new(n);
        qs.into_iter()
            .filter_map(|q| match q {
                Query::Unite(x, y) => {
                    ds.unite(x, y);
                    None
                }
                Query::Same(x, y) => Some(ds.equiv(x, y)),
            })
            .collect()
    }
}
