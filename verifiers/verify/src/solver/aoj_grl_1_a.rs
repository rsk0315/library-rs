use crate::jury;
use crate::test_set::{Jury, Solver};

use dijkstra_::dijkstra;

pub struct AojGrl1A {}

impl Solver for AojGrl1A {
    type Jury = jury::AojGrl1A;
    fn solve((n, r, es): <Self::Jury as Jury>::Input) -> Vec<Option<i32>> {
        let g = {
            let mut g: Vec<_> = vec![vec![]; n];
            for (u, v, w) in es {
                g[u].push((v, w));
            }
            g
        };

        let index = |&i: &usize| -> usize { i };
        let delta = |&v: &usize| g[v].iter().cloned();
        dijkstra(n, r, 0, index, delta)
    }
}
