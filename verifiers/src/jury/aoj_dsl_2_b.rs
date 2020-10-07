use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::*;

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
        let mut input = input.lines();
        let (n, q) = {
            let mut it = input.next().unwrap().split(" ");
            let n = it.next().unwrap().parse().unwrap();
            let q = it.next().unwrap().parse().unwrap();
            (n, q)
        };
        let qs = (0..q)
            .map(|_| {
                let mut it = input.next().unwrap().split(" ");
                match it.next().unwrap() {
                    "0" => {
                        let i =
                            it.next().unwrap().parse::<usize>().unwrap() - 1;
                        let x = it.next().unwrap().parse().unwrap();
                        Query::Add(i, x)
                    }
                    "1" => {
                        let s =
                            it.next().unwrap().parse::<usize>().unwrap() - 1;
                        let t = it.next().unwrap().parse().unwrap();
                        Query::GetSum(s, t)
                    }
                    _ => unreachable!(),
                }
            })
            .collect();
        (n, qs)
    }
    fn parse_output((_n, qs): &Self::Input, output: String) -> Self::Output {
        let mut output = output.lines();
        qs.iter()
            .filter_map(|q| match q {
                Query::Add(_, _) => None,
                Query::GetSum(_, _) => {
                    Some(output.next().unwrap().parse().unwrap())
                }
            })
            .collect()
    }
}
