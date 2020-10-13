use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{Jury, Oj, Yukicoder};

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
        let mut input = input.lines();
        let (_n, q) = {
            let mut tmp =
                input.next().unwrap().split(' ').map(|s| s.parse().unwrap());

            let n = tmp.next().unwrap();
            let q = tmp.next().unwrap();
            (n, q)
        };

        let a = input
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let qs = input
            .take(q)
            .map(|ss| {
                let mut ss = ss.split(' ');
                match ss.next().unwrap() {
                    "1" => {
                        let l =
                            ss.next().unwrap().parse::<usize>().unwrap() - 1;
                        let r = ss.next().unwrap().parse().unwrap();
                        Query::Type1(l, r)
                    }
                    _ => unreachable!(),
                }
            })
            .collect();
        (a, qs)
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let q = input.1.len();
        output.lines().take(q).map(|s| s.parse().unwrap()).collect()
    }
}
