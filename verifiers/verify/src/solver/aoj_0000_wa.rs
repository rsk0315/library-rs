use crate::jury;
use crate::test_set::Solver;

pub struct Aoj0000Wa {}

impl Solver for Aoj0000Wa {
    type Jury = jury::Aoj0000;
    fn solve(_: ()) -> Vec<String> {
        vec!["hello world".to_string()]
    }
}
