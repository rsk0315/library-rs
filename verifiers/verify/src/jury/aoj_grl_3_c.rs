use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct AojGrl3C {}

impl Jury for AojGrl3C {
    type Input = (usize, Vec<(usize, usize)>, Vec<(usize, usize)>);
    type Output = Vec<bool>;
    const TL: Duration = Duration::from_millis(1000);
    const PROBLEM: Oj = Aoj("GRL_3_C");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, m) = input.next().unwrap();
        let es = input.next_n(m).unwrap();

        let q = input.next().unwrap();
        let qs = input.next_n(q).unwrap();
        (n, es, qs)
    }
    fn parse_output((_, _, qs): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();
        let q = qs.len();

        (0..q)
            .map(|_| match output.get_line().trim() {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            })
            .collect()
    }
}
