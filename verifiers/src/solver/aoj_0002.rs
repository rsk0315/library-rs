use crate::jury;
use crate::test_set::*;

pub struct Aoj0002 {}

impl Solver for Aoj0002 {
    type Jury = jury::Aoj0002;
    fn solve(input: Vec<(u32, u32)>) -> Vec<usize> {
        input
            .into_iter()
            .map(|(a, b)| (a + b).to_string().len())
            .collect()
    }
}
