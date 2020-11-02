use crate::jury;
use crate::test_set::{Jury, Solver};

use scc::scc;

pub struct AojGrl3C {}

impl Solver for AojGrl3C {
    type Jury = jury::AojGrl3C;
    fn solve((n, es, qs): <Self::Jury as Jury>::Input) -> Vec<bool> {
        let g = {
            let mut g: Vec<_> = vec![vec![]; n];
            for (u, v) in es {
                g[u].push(v);
            }
            g
        };

        let index = |&i: &usize| -> usize { i };
        let delta = |&v: &usize, f: &mut dyn FnMut(usize)| {
            g[v].iter().for_each(|&nv| f(nv));
        };
        let scc_id = scc(n, 0..n, index, delta);
        qs.into_iter()
            .map(|(u, v)| scc_id[u] == scc_id[v])
            .collect()
    }
}
