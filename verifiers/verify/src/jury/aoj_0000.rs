use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct Aoj0000 {}

impl Jury for Aoj0000 {
    type Input = ();
    type Output = Vec<String>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("0000");
    fn parse_input(_: String) -> Self::Input {}
    fn parse_output(_: &(), output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        output.next_n(81).unwrap()
    }
}
