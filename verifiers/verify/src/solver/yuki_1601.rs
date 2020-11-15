use std::ops::Bound::{Excluded, Included};
use std::ops::RangeInclusive;

use interval_set::IntervalSet;

use crate::jury;
use crate::test_set::Solver;

pub struct Yuki1601 {}

impl Solver for Yuki1601 {
    type Jury = jury::Yuki1601;
    fn solve((_d, qs): (u64, Vec<RangeInclusive<u64>>)) -> Vec<u64> {
        let mut is = IntervalSet::<u64>::new();
        let mut res = 0;
        qs.into_iter()
            .map(|q| {
                let (s, e) = q.into_inner();
                is.insert(s..e + 1);
                res = match is.covering(&(s..=s)) {
                    Some((Included(s), Excluded(e))) => res.max(e - s),
                    _ => unreachable!(),
                };
                res
            })
            .collect()
    }
}
