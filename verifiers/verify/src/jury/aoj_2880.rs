use std::ops::RangeInclusive;
use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct Aoj2880 {}

impl Jury for Aoj2880 {
    type Input = (
        u32,
        Vec<(u32, RangeInclusive<u32>)>,
        Vec<(u32, RangeInclusive<u32>)>,
    );
    type Output = Vec<bool>;
    const TL: Duration = Duration::from_millis(2000);
    const PROBLEM: Oj = Aoj("2880");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, m, q) = input.next().unwrap();
        let dab = (0..m).map(|_| {
            let (d, a, b) = input.next().unwrap();
            (d, a..=b)
        });
        let dab = dab.collect();
        let est = (0..q).map(|_| {
            let (e, s, t) = input.next().unwrap();
            (e, s..=t)
        });
        let est = est.collect();

        (n, dab, est)
    }
    fn parse_output((_, _, qs): &Self::Input, output: String) -> Self::Output {
        let q = qs.len();
        let mut output: Scanner = output.into();

        (0..q)
            .map(|_| match output.get_line().trim() {
                "Yes" => true,
                "No" => false,
                _ => unreachable!(),
            })
            .collect()
    }
}
