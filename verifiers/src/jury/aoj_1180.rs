use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

pub struct Aoj1180 {}

impl Jury for Aoj1180 {
    type Input = Vec<(u32, usize)>;
    type Output = Vec<(usize, u32, usize)>;
    const TL: Duration = Duration::from_millis(8000);
    const PROBLEM: Oj = Aoj("1180");
    fn parse_input(input: String) -> Self::Input {
        input
            .lines()
            .filter_map(|s| {
                let mut it = s.split(' ');
                let a = it.next().unwrap().parse().unwrap();
                let l = it.next().unwrap().parse().unwrap();
                match (a, l) {
                    (0, 0) => None,
                    (a, l) => Some((a, l)),
                }
            })
            .collect()
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let mut output = output.lines();
        let n = input.len();
        (0..n)
            .map(|_| {
                let mut it = output.next().unwrap().split(' ');
                let mu = it.next().unwrap().parse().unwrap();
                let a = it.next().unwrap().parse().unwrap();
                let lambda = it.next().unwrap().parse().unwrap();
                (mu, a, lambda)
            })
            .collect()
    }
}
