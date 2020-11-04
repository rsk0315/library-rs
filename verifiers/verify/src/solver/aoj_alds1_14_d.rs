use crate::jury;
use crate::test_set::Solver;

use suffix_array::SuffixArray;

pub struct AojAldsOne14D {}

impl Solver for AojAldsOne14D {
    type Jury = jury::AojAldsOne14D;
    fn solve((t, p): (String, Vec<String>)) -> Vec<bool> {
        let sa = SuffixArray::from(&t);
        p.into_iter().map(|p| !sa.search(&p).is_empty()).collect()
    }
}
