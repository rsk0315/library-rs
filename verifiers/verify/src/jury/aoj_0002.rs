use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use parser::Parser;

pub struct Aoj0002 {}

impl Jury for Aoj0002 {
    type Input = Vec<(u32, u32)>;
    type Output = Vec<usize>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("0002");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Parser = input.into();

        std::iter::repeat(0)
            .map(|_| {
                let a = input.next();
                let b = input.next();
                (a, b)
            })
            .take_while(|(_, b)| b.is_ok())
            .map(|(a, b)| (a.unwrap(), b.unwrap()))
            .collect()
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let n = input.len();
        let mut output: Parser = output.into();

        output.next_n(n).map(std::result::Result::unwrap).collect()
    }
}
