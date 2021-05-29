use std::cell::RefCell;
use std::rc::Rc;

use crate::jury;
use crate::test_set::{Jury, Solver};

use dinic_::dinic;

pub struct AojGrl6A {}

impl Solver for AojGrl6A {
    type Jury = jury::AojGrl6A;
    fn solve((n, es): <Self::Jury as Jury>::Input) -> i32 {
        let g = {
            let mut g = vec![vec![]; n];
            for (from, to, capacity) in es {
                let from_len = g[from].len();
                let to_len = g[to].len();
                g[from].push((to, Rc::new(RefCell::new(capacity)), to_len));
                g[to].push((from, Rc::new(RefCell::new(0)), from_len));
            }
            g
        };

        let index = |&v: &usize| -> usize { v };
        let delta =
            |&v: &usize| g[v].iter().map(|&(nv, ref w, r)| (nv, w.clone(), r));
        let rev = |&nv: &usize, &r: &usize| g[nv][r].1.clone();

        let s = 0;
        let t = n - 1;
        dinic(n, s, t, 0..n, 0, index, delta, rev)
    }
}
