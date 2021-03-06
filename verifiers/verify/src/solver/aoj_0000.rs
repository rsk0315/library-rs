use crate::jury;
use crate::test_set::Solver;

pub struct Aoj0000 {}

impl Solver for Aoj0000 {
    type Jury = jury::Aoj0000;
    fn solve(_: ()) -> Vec<String> {
        (1..=9)
            .flat_map(|i| {
                (1..=9).map(move |j| format!("{}x{}={}", i, j, i * j))
            })
            .collect::<Vec<_>>()
    }
}
