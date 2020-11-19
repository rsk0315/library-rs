use std::ops::Range;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{judge_vec, Aoj, Jury, Oj, Verdict};

use scanner::Scanner;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Update(Range<usize>, u32),
    Find(usize),
}

pub struct AojDsl2D {}

impl Jury for AojDsl2D {
    type Input = (usize, Vec<Query>);
    type Output = Vec<u32>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("DSL_2_D");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, q) = input.next().unwrap();

        let qs = (0..q).map(|_| match input.next().unwrap() {
            0 => {
                let s = input.next().unwrap();
                let t = input.next::<usize>().unwrap() + 1;
                let x = input.next().unwrap();
                Query::Update(s..t, x)
            }
            1 => {
                let i = input.next().unwrap();
                Query::Find(i)
            }
            _ => unreachable!(),
        });
        (n, qs.collect())
    }
    fn parse_output((_n, qs): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        qs.iter()
            .filter_map(|q| match q {
                Query::Update(_, _) => None,
                Query::Find(_) => Some(output.next().unwrap()),
            })
            .collect()
    }
    fn judge(
        _: Self::Input,
        output: Self::Output,
        jury: Self::Output,
    ) -> Verdict {
        judge_vec(&output, &jury)
    }
}
