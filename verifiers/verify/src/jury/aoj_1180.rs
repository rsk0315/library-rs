use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use parser::Parser;

pub struct Aoj1180 {}

impl Jury for Aoj1180 {
    type Input = Vec<(u32, usize)>;
    type Output = Vec<(usize, u32, usize)>;
    const TL: Duration = Duration::from_millis(8000);
    const PROBLEM: Oj = Aoj("1180");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Parser = input.into();

        std::iter::repeat(0)
            .map(|_| {
                let a = input.next().unwrap();
                let l = input.next().unwrap();
                match (a, l) {
                    (0, 0) => None,
                    (a, l) => Some((a, l)),
                }
            })
            .fuse()
            .map(std::option::Option::unwrap)
            .collect()
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let mut output: Parser = output.into();

        let n = input.len();
        (0..n)
            .map(|_| {
                let mu = output.next().unwrap();
                let a = output.next().unwrap();
                let lambda = output.next().unwrap();
                (mu, a, lambda)
            })
            .collect()
    }
}
