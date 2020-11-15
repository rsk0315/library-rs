use std::ops::RangeInclusive;
use std::time::Duration;

use crate::test_set::{Jury, Oj, Yukicoder};

use scanner::Scanner;

pub struct Yuki1601 {}

impl Jury for Yuki1601 {
    type Input = (u64, Vec<RangeInclusive<u64>>);
    type Output = Vec<u64>;
    const TL: Duration = Duration::from_millis(2000);
    const PROBLEM: Oj = Yukicoder("1601");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (d, q) = input.next().unwrap();

        let ab = (0..q).map(|_| {
            let (a, b) = input.next().unwrap();
            a..=b
        });
        let ab = ab.collect();

        (d, ab)
    }
    fn parse_output((_, qs): &Self::Input, output: String) -> Self::Output {
        let q = qs.len();
        let mut output: Scanner = output.into();

        output.next_n(q).unwrap()
    }
}
