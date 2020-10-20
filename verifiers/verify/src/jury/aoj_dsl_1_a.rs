use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Unite(usize, usize),
    Same(usize, usize),
}

pub struct AojDsl1A {}

impl Jury for AojDsl1A {
    type Input = (usize, Vec<Query>);
    type Output = Vec<bool>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("DSL_1_A");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, q) = input.next().unwrap();

        let qs = (0..q).map(|_| match input.next().unwrap() {
            0 => {
                let (x, y) = input.next().unwrap();
                Query::Unite(x, y)
            }
            1 => {
                let (x, y) = input.next().unwrap();
                Query::Same(x, y)
            }
            _ => unreachable!(),
        });
        (n, qs.collect())
    }
    fn parse_output((_n, qs): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        qs.iter()
            .filter_map(|q| match q {
                Query::Unite(_, _) => None,
                Query::Same(_, _) => Some(match output.next().unwrap() {
                    0 => false,
                    1 => true,
                    _ => unreachable!(),
                }),
            })
            .collect()
    }
}
