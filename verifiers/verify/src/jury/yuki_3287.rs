use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{Jury, Oj, Yukicoder};

use parser::Parser;

pub struct Yuki3287 {}

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Type1(usize, usize),
}

impl Jury for Yuki3287 {
    type Input = (Vec<u32>, Vec<Query>);
    type Output = Vec<usize>;
    const TL: Duration = Duration::from_millis(2000);
    const PROBLEM: Oj = Yukicoder("3287");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Parser = input.into();

        let n = input.next().unwrap();
        let q = input.next().unwrap();

        let a = input.next_n(n).map(std::result::Result::unwrap).collect();
        let qs = (0..q).map(|_| match input.next().unwrap() {
            '1' => {
                let l = input.next::<usize>().unwrap() - 1;
                let r = input.next().unwrap();
                Query::Type1(l, r)
            }
            _ => unreachable!(),
        });
        let qs = qs.collect();

        (a, qs)
    }
    fn parse_output((_, qs): &Self::Input, output: String) -> Self::Output {
        let q = qs.len();
        let mut output: Parser = output.into();

        output.next_n(q).map(std::result::Result::unwrap).collect()
    }
}
