use std::ops::RangeInclusive;

use crate::jury;
use crate::test_set::Solver;

use interval_set::IntervalSet;

pub struct Aoj2880 {}

impl Solver for Aoj2880 {
    type Jury = jury::Aoj2880;
    fn solve(
        (_n, dab, est): (
            u32,
            Vec<(u32, RangeInclusive<u32>)>,
            Vec<(u32, RangeInclusive<u32>)>,
        ),
    ) -> Vec<bool> {
        let q = est.len();

        // (day, qi, range)
        let dab = dab
            .into_iter()
            .map(|(d, ab)| ((d, 1), 0, (*ab.start(), *ab.end())));
        let est = est
            .into_iter()
            .enumerate()
            .map(|(i, (e, st))| ((e, 0), i, (*st.start(), *st.end())));
        let mut qs: Vec<_> = dab.chain(est).collect();
        qs.sort_unstable();

        let mut is = IntervalSet::<u32>::new();
        let mut res = vec![false; q];

        for ((_, d), qi, (rs, re)) in qs {
            match d {
                0 => res[qi] = rs >= re || is.has_range(&(rs..=re)),
                1 => is.insert(rs..=re),
                _ => unreachable!(),
            }
        }
        res
    }
}
