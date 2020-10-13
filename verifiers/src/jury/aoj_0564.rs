use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

pub struct Aoj0564 {}

impl Jury for Aoj0564 {
    type Input = Vec<(u64, u64)>;
    type Output = u64;
    const TL: Duration = Duration::from_millis(8000);
    const PROBLEM: Oj = Aoj("0564");
    fn parse_input(input: String) -> Self::Input {
        let mut input = input.lines();
        let n = input.next().unwrap().parse::<usize>().unwrap();
        (0..n)
            .map(|_| {
                let mut it = input
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|x| x.parse().unwrap());
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                (a, b)
            })
            .collect()
    }
    fn parse_output(_: &Self::Input, output: String) -> Self::Output {
        output.lines().next().unwrap().parse().unwrap()
    }
}
