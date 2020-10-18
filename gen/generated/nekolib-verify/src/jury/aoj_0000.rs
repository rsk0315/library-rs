use std::time::Duration;

use crate::test_set::{Jury, Oj};

pub struct Aoj0000 {}

impl Jury for Aoj0000 {
    type Input = ();
    type Output = Vec<String>;
    const TL: Duration = Duration::from_millis(10000);
    const PROBLEM: Oj = Oj::Aoj("0000");
    fn parse_input(_: String) -> Self::Input {}
    fn parse_output(_: &Self::Input, output: String) -> Self::Output {
        output
            .lines()
            .take(9 * 9)
            .map(std::string::ToString::to_string)
            .collect()
    }
}
