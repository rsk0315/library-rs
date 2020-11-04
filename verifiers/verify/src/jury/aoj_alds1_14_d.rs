use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct AojAldsOne14D {}

impl Jury for AojAldsOne14D {
    type Input = (String, Vec<String>);
    type Output = Vec<bool>;
    const TL: Duration = Duration::from_millis(3000);
    const PROBLEM: Oj = Aoj("ALDS1_14_D");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let t = input.next().unwrap();
        let q = input.next().unwrap();
        let p = input.next_n(q).unwrap();

        (t, p)
    }
    fn parse_output((_, p): &Self::Input, output: String) -> Self::Output {
        let mut output: Scanner = output.into();

        p.into_iter()
            .map(|_| match output.get_line().trim() {
                "0" => false,
                "1" => true,
                _ => unreachable!(),
            })
            .collect()
    }
}
