use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct AojGrl6A {}

impl Jury for AojGrl6A {
    type Input = (usize, Vec<(usize, usize, i32)>);
    type Output = i32;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("GRL_6_A");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, m) = input.next().unwrap();
        let es = input.next_n(m).unwrap();
        (n, es)
    }
    fn parse_output(_: &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();
        output.next().unwrap()
    }
}
