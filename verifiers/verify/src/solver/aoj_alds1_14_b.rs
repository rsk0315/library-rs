use crate::jury;
use crate::test_set::Solver;

use suffix_array::SuffixArray;

pub struct AojAldsOne14B {}

impl Solver for AojAldsOne14B {
    type Jury = jury::AojAldsOne14B;
    fn solve((t, p): (String, String)) -> Vec<usize> {
        let t: Vec<_> = t.as_str().chars().collect();
        let p: Vec<_> = p.as_str().chars().collect();
        let sa = SuffixArray::from(t);
        let mut res: Vec<_> = sa.search(&p).collect();
        res.sort_unstable();
        res
    }
}
