use std::time::Duration;

use crate::test_set::{Aoj, Jury, Oj};

use scanner::Scanner;

pub struct Aoj0575 {}

impl Jury for Aoj0575 {
    type Input = (
        usize,
        Vec<(usize, usize, i32)>,
        Vec<usize>,
        Vec<(usize, usize)>,
    );
    type Output = Vec<i32>;
    const TL: Duration = Duration::from_millis(8000);
    const PROBLEM: Oj = Aoj("0575");
    fn parse_input(input: String) -> Self::Input {
        let mut input: Scanner = input.into();

        let (n, k, m, q) = input.next().unwrap();
        let abl = (0..k).map(|_| {
            let a = input.next_m1().unwrap();
            let b = input.next_m1().unwrap();
            let l = input.next().unwrap();
            (a, b, l)
        });
        let abl = abl.collect();
        let f = (0..m).map(|_| input.next_m1().unwrap()).collect();
        let qs = (0..q).map(|_| {
            let s = input.next_m1().unwrap();
            let t = input.next_m1().unwrap();
            (s, t)
        });
        let qs = qs.collect();

        (n, abl, f, qs)
    }
    fn parse_output(
        (_, _, _, qs): &Self::Input,
        output: String,
    ) -> Self::Output {
        let mut output: Scanner = output.into();
        let q = qs.len();

        output.next_n(q).unwrap()
    }
}
