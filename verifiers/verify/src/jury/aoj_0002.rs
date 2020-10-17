use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

pub struct Aoj0002 {}

impl Jury for Aoj0002 {
    type Input = Vec<(u32, u32)>;
    type Output = Vec<usize>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("0002");
    fn parse_input(input: String) -> Self::Input {
        input
            .lines()
            .take_while(|s| !s.is_empty())
            .map(|s| {
                let mut it = s.split(' ').map(|x| x.parse().unwrap());
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                (a, b)
            })
            .collect()
    }
    fn parse_output(input: &Self::Input, output: String) -> Self::Output {
        let n = input.len();
        output.lines().take(n).map(|s| s.parse().unwrap()).collect()
    }
}
