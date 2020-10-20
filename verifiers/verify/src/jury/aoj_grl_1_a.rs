use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct AojGrl1A {}

impl Jury for AojGrl1A {
    type Input = (usize, usize, Vec<(usize, usize, i32)>);
    type Output = Vec<Option<i32>>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("GRL_1_A");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, m, r) = input.next().unwrap();
        let es = input.next_n(m).unwrap();
        (n, r, es)
    }
    fn parse_output(&(n, _, _): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        (0..n)
            .map(|_| match output.get_line().trim() {
                "INF" => None,
                s => Some(s.parse().unwrap()),
            })
            .collect()
    }
}
