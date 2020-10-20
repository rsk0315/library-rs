use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct Aoj0270 {}

impl Jury for Aoj0270 {
    type Input = (Vec<u32>, Vec<u32>);
    type Output = Vec<u32>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("0270");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, q) = input.next().unwrap();
        let c = input.next_n(n).unwrap();
        let qs = input.next_n(q).unwrap();
        (c, qs)
    }
    fn parse_output((_, qs): &Self::Input, output: String) -> Self::Output {
        let q = qs.len();
        let mut output: Scanner = output.into();

        output.next_n(q).unwrap()
    }
}
