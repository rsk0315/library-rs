use tortoise_hare::tortoise_hare;

use crate::jury;
use crate::test_set::Solver;

pub struct Aoj1180 {}

impl Solver for Aoj1180 {
    type Jury = jury::Aoj1180;
    fn solve(al: Vec<(u32, usize)>) -> Vec<(usize, u32, usize)> {
        al.into_iter()
            .map(|(a, l)| {
                let f = |a| {
                    let s = format!("{0:01$}", a, l);
                    let mut s: Vec<_> = s.chars().collect();
                    s.sort_unstable();
                    let s0: u32 = s.iter().collect::<String>().parse().unwrap();
                    let s1: u32 =
                        s.iter().rev().collect::<String>().parse().unwrap();
                    s1 - s0
                };
                let (mu, lambda) = tortoise_hare(a, f);
                let a = std::iter::successors(Some(a), |&x| Some(f(x)))
                    .nth(mu)
                    .unwrap();
                (mu, a, lambda)
            })
            .collect()
    }
}
