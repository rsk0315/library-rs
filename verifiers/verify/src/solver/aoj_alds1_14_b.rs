use crate::jury;
use crate::test_set::Solver;

use suffix_array::SuffixArray;

pub struct AojAldsOne14B {}

impl Solver for AojAldsOne14B {
    type Jury = jury::AojAldsOne14B;
    fn solve((t, p): (String, String)) -> Vec<usize> {
        let sa: SuffixArray<_> = t.into();
        let mut res: Vec<_> = sa.search_str(&p).collect();
        res.sort_unstable();
        res
    }
}
