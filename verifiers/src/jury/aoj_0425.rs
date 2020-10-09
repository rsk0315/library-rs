use std::fmt::Debug;
use std::ops::Range;
use std::time::Duration;

use crate::test_set::*;

use serde::{Deserialize, Serialize};

pub struct Aoj0425 {}

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

impl Jury for Aoj0425 {
    type Input = (usize, Vec<(usize, usize)>, Vec<(Range<usize>, Query)>);
    type Output = Vec<Response>;
    const TL: Duration = Duration::from_millis(2000);
    const PROBLEM: Oj = Aoj("0425");
    fn parse_input(input: String) -> Self::Input {
        let mut input = input.lines();
        let (n, k, q) = {
            let mut it = input.next().unwrap().split(' ');
            let n = it.next().unwrap().parse().unwrap();
            let k = it.next().unwrap().parse().unwrap();
            let q = it.next().unwrap().parse().unwrap();
            (n, k, q)
        };
        let ab = (0..k).map(|_| {
            let mut it = input.next().unwrap().split(' ');
            let a = it.next().unwrap().parse::<usize>().unwrap() - 1;
            let b = it.next().unwrap().parse::<usize>().unwrap() - 1;
            (a, b)
        });
        let ab = ab.collect();
        let qs = (0..q).map(|_| {
            let mut it = input.next().unwrap().split(' ');
            match it.next().unwrap() {
                "1" => {
                    let s = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    let t = it.next().unwrap().parse().unwrap();
                    let x = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    (s..t, Query::Type1(x))
                }
                "2" => {
                    let s = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    let t = it.next().unwrap().parse().unwrap();
                    let x = it.next().unwrap().parse::<usize>().unwrap() - 1;
                    (s..t, Query::Type2(x))
                }
                _ => unreachable!(),
            }
        });
        let qs = qs.collect();
        (n, ab, qs)
    }
    fn parse_output((_, _, qs): &Self::Input, output: String) -> Self::Output {
        let mut output = output.lines();
        qs.iter()
            .filter_map(|q| {
                let r = output.next().unwrap().parse::<usize>().unwrap() - 1;
                match q {
                    (_, Query::Type1(_)) => Some(Response::Type1(r)),
                    (_, Query::Type2(_)) => Some(Response::Type2(r)),
                }
            })
            .collect()
    }
}
