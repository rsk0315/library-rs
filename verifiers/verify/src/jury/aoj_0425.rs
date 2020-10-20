use std::fmt::Debug;
use std::ops::Range;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Query {
    Type1(usize),
    Type2(usize),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Response {
    Type1(usize),
    Type2(usize),
}

pub struct Aoj0425 {}

impl Jury for Aoj0425 {
    type Input = (usize, Vec<(usize, usize)>, Vec<(Range<usize>, Query)>);
    type Output = Vec<Response>;
    const TL: Duration = Duration::from_millis(2000);
    const PROBLEM: Oj = Aoj("0425");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, k, q) = input.next().unwrap();

        let ab = (0..k).map(|_| {
            let a = input.next_m1().unwrap();
            let b = input.next_m1().unwrap();
            (a, b)
        });
        let ab = ab.collect();

        let qs = (0..q).map(|_| match input.next().unwrap() {
            1 => {
                let s = input.next_m1().unwrap();
                let t = input.next().unwrap();
                let x = input.next_m1().unwrap();
                (s..t, Query::Type1(x))
            }
            2 => {
                let s = input.next_m1().unwrap();
                let t = input.next().unwrap();
                let x = input.next_m1().unwrap();
                (s..t, Query::Type2(x))
            }
            _ => unreachable!(),
        });
        let qs = qs.collect();
        (n, ab, qs)
    }
    fn parse_output((_, _, qs): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();
        qs.iter()
            .map(|q| {
                let r = output.next_m1().unwrap();
                match q {
                    (_, Query::Type1(_)) => Response::Type1(r),
                    (_, Query::Type2(_)) => Response::Type2(r),
                }
            })
            .collect()
    }
}
