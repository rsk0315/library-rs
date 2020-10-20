use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct Aoj0002 {}

impl Jury for Aoj0002 {
    type Input = Vec<(u32, u32)>;
    type Output = Vec<usize>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("0002");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        std::iter::repeat(0)
            .map(|_| input.next())
            .take_while(std::result::Result::is_ok)
            .map(std::result::Result::unwrap)
            .collect()
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let n = input.len();
        let mut output: Scanner = output.into();

        output.next_n(n).unwrap()
    }
}
