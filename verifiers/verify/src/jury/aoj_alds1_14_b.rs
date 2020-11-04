use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct AojAldsOne14B {}

impl Jury for AojAldsOne14B {
    type Input = (String, String);
    type Output = Vec<usize>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("ALDS1_14_B");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let t = input.next().unwrap();
        let p = input.next().unwrap();

        (t, p)
    }
    fn parse_output(_: &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        (0..)
            .map(|_| output.next())
            .take_while(std::result::Result::is_ok)
            .map(std::result::Result::unwrap)
            .collect()
    }
}
