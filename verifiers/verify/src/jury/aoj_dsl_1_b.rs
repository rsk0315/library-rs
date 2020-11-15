use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Relate(usize, usize, i32),
    Diff(usize, usize),
}

pub struct AojDsl1B {}

impl Jury for AojDsl1B {
    type Input = (usize, Vec<Query>);
    type Output = Vec<Option<i32>>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("DSL_1_B");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, q) = input.next().unwrap();

        let qs = (0..q).map(|_| match input.next().unwrap() {
            0 => {
                let (x, y, z) = input.next().unwrap();
                Query::Relate(x, y, z)
            }
            1 => {
                let (x, y) = input.next().unwrap();
                Query::Diff(x, y)
            }
            _ => unreachable!(),
        });
        (n, qs.collect())
    }
    fn parse_output((_n, qs): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        qs.iter()
            .filter_map(|q| match q {
                Query::Relate(_, _, _) => None,
                Query::Diff(_, _) => Some(match output.get_line().trim() {
                    "?" => None,
                    z => Some(z.parse().unwrap()),
                }),
            })
            .collect()
    }
}
