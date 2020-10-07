use crate::jury;
use crate::test_set::*;

pub struct Aoj0000Tle {}

impl Solver for Aoj0000Tle {
    type Jury = jury::Aoj0000;
    fn solve(_: ()) -> Vec<String> {
        let n = 100000000;
        vec![(1..=n)
            .map(|i| (1..=i).step_by(2).sum::<u128>())
            .sum::<u128>()
            .to_string()]
    }
}
