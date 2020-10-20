use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct Aoj0564 {}

impl Jury for Aoj0564 {
    type Input = Vec<(u64, u64)>;
    type Output = u64;
    const TL: Duration = Duration::from_millis(8000);
    const PROBLEM: Oj = Aoj("0564");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let n = input.next().unwrap();
        input.next_n(n).unwrap()
    }
    fn parse_output(_: &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        output.next().unwrap()
    }
}
