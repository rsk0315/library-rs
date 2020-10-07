use std::marker::PhantomData;
use std::ops::{Index, Range};

use fold::Fold;
use fold_bisect::FoldBisectRev;
use op_add::OpAdd;
use op_max::OpMax;
use set_value::SetValue;

use crate::jury;
use crate::test_set::*;

use jury::yuki_3287::Query;

pub struct Yuki3287<D1, D2>
where
    D1: From<Vec<u32>>
        + Index<usize, Output = u32>
        + FoldBisectRev<Folded = OpMax<u32>>,
    D2: From<Vec<usize>>
        + SetValue<usize, Input = usize>
        + Fold<Range<usize>, Output = OpAdd<usize>>,
{
    _d1: PhantomData<D1>,
    _d2: PhantomData<D2>,
}

impl<D1, D2> Solver for Yuki3287<D1, D2>
where
    D1: From<Vec<u32>>
        + Index<usize, Output = u32>
        + FoldBisectRev<Folded = OpMax<u32>>,
    D2: From<Vec<usize>>
        + SetValue<usize, Input = usize>
        + Fold<Range<usize>, Output = OpAdd<usize>>,
{
    type Jury = jury::Yuki3287;
    fn solve((a, qs): (Vec<u32>, Vec<Query>)) -> Vec<usize> {
        let n = a.len();
        let q = qs.len();

        let top: Vec<_> = {
            let rq: D1 = a.into();
            (0..n)
                .map(|i| match rq.fold_bisect_rev(i + 1, |x| x <= &rq[i]) {
                    Some(l) => l + 1,
                    None => 0,
                })
                .collect()
        };

        let js = {
            let mut tmp = vec![vec![]; n];
            for i in 0..n {
                tmp[top[i]].push(i);
            }
            tmp
        };

        let qs = {
            let mut tmp = vec![vec![]; n];
            for (iq, q) in qs.into_iter().enumerate() {
                match q {
                    Query::Type1(l, r) => tmp[l].push(((l, r), iq)),
                }
            }
            tmp
        };

        let mut rq: D2 = vec![0; n].into();
        let mut res = vec![0; q];

        for i in 0..n {
            for &j in &js[i] {
                rq.set_value(j, 1);
            }
            for &((l, r), iq) in &qs[i] {
                res[iq] = rq.fold(l..r);
            }
        }

        res
    }
}
