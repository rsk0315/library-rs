use std::time::Duration;

use crate::test_set::*;

pub struct Aoj0270 {}

impl Jury for Aoj0270 {
    type Input = (Vec<u32>, Vec<u32>);
    type Output = Vec<u32>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("0270");
    fn parse_input(input: String) -> Self::Input {
        let mut input = input.lines();
        let (_n, q) = {
            let mut it = input.next().unwrap().split(" ");
            let n = it.next().unwrap().parse::<usize>().unwrap();
            let q = it.next().unwrap().parse().unwrap();
            (n, q)
        };
        let c = input
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();
        let qs = (0..q)
            .map(|_| input.next().unwrap().parse().unwrap())
            .collect();
        (c, qs)
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let q = input.1.len();
        let mut output = output.lines();
        (0..q)
            .map(|_| output.next().unwrap().parse().unwrap())
            .collect()
    }
}
