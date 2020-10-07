use std::time::Duration;

use crate::test_set::*;

pub struct Aoj0000 {}

impl Jury for Aoj0000 {
    type Input = ();
    type Output = Vec<String>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("0000");
    fn parse_input(_: String) -> Self::Input {
        ()
    }
    fn parse_output(_: &(), output: String) -> Self::Output {
        output.lines().take(9 * 9).map(|s| s.to_string()).collect()
    }
}
