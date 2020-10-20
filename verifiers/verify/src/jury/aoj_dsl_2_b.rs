use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Add(usize, u64),
    GetSum(usize, usize),
}

pub struct AojDsl2B {}

impl Jury for AojDsl2B {
    type Input = (usize, Vec<Query>);
    type Output = Vec<u64>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("DSL_2_B");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, q) = input.next().unwrap();

        let qs = (0..q).map(|_| match input.next().unwrap() {
            0 => {
                let i = input.next_m1().unwrap();
                let x = input.next().unwrap();
                Query::Add(i, x)
            }
            1 => {
                let s = input.next_m1().unwrap();
                let t = input.next().unwrap();
                Query::GetSum(s, t)
            }
            _ => unreachable!(),
        });
        (n, qs.collect())
    }
    fn parse_output((_n, qs): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        qs.iter()
            .filter_map(|q| match q {
                Query::Add(_, _) => None,
                Query::GetSum(_, _) => Some(output.next().unwrap()),
            })
            .collect()
    }
}
