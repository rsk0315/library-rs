use crate::jury;
use crate::test_set::Solver;

use kmp::KmpSearcher;

pub struct AojAldsOne14BKmp {}

impl Solver for AojAldsOne14BKmp {
    type Jury = jury::AojAldsOne14B;
    fn solve((t, p): (String, String)) -> Vec<usize> {
        let t: Vec<_> = t.as_str().bytes().collect();
        let p: Vec<_> = p.as_str().bytes().collect();
        let pat: KmpSearcher<u8> = p.into();
        pat.occurrences(&t).into_iter().map(|r| r.start).collect()
    }
}
