use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use parser::Parser;

pub struct Aoj0270 {}

impl Jury for Aoj0270 {
    type Input = (Vec<u32>, Vec<u32>);
    type Output = Vec<u32>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("0270");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Parser = input.into();

        let n = input.next().unwrap();
        let q = input.next().unwrap();
        let c = input.next_n(n).map(std::result::Result::unwrap).collect();
        let qs = input.next_n(q).map(std::result::Result::unwrap).collect();
        (c, qs)
    }
    fn parse_output((_, qs): &Self::Input, output: String) -> Self::Output {
        let q = qs.len();
        let mut output: Parser = output.into();

        output.next_n(q).map(std::result::Result::unwrap).collect()
    }
}
