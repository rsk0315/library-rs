use std::marker::PhantomData;

use std::collections::{BTreeMap, BTreeSet};

use crate::jury;
use crate::test_set::{Jury, Solver};

use dijkstra::dijkstra;
use disjoint_set::DisjointSet;
use parallel_bisect::parallel_bisect;
use stateful_predicate::StatefulPred;

pub struct Aoj0575<D> {
    _d: PhantomData<D>,
}

impl<D> Solver for Aoj0575<D>
where
    D: DisjointSet,
{
    type Jury = jury::Aoj0575;
    fn solve((n, abl, f, q): <Self::Jury as Jury>::Input) -> Vec<i32> {
        let g = {
            let mut g = vec![vec![]; n + 1];
            for (a, b, l) in abl {
                g[a].push((b, l));
                g[b].push((a, l));
            }
            g[n] = f.into_iter().map(|v| (v, 0)).collect();
            g
        };

        let index = |&i: &usize| -> usize { i };
        let delta = |&v: &usize, f: &mut dyn FnMut(usize, i32)| {
            for &(nv, ew) in &g[v] {
                f(nv, ew);
            }
        };

        let dist: Vec<_> = dijkstra(n + 1, n, 0, index, delta)
            .into_iter()
            .map(std::option::Option::unwrap)
            .collect();

        let enc: BTreeSet<_> = dist.iter().cloned().collect();
        let enc: BTreeMap<_, _> =
            enc.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
        let dec: Vec<_> = enc.keys().cloned().collect();
        let m = dec.len();

        let es = {
            let mut es = vec![vec![]; m];
            for u in 0..n {
                for &(v, w) in &g[u] {
                    let iu = enc[&dist[u]];
                    if dist[u] + w <= dist[v] {
                        es[iu].push((u, v));
                    }
                    if dist[u] <= dist[v] {
                        es[iu].push((u, v));
                    }
                }
            }
            es
        };

        parallel_bisect(Neko::<D>::new(n, es), q)
            .into_iter()
            .map(|i| dec[m - i])
            .collect()
    }
}

struct Neko<D: DisjointSet> {
    ds: D,
    es: Vec<Vec<(usize, usize)>>,
    i: usize,
}

impl<D: DisjointSet> Neko<D> {
    pub fn new(n: usize, es: Vec<Vec<(usize, usize)>>) -> Self {
        let i = es.len();
        Self {
            ds: D::new(n),
            es,
            i,
        }
    }
}

impl<D: DisjointSet> StatefulPred for Neko<D> {
    type Input = (usize, usize);
    fn reset(&mut self) {
        let n = self.ds.len();
        self.ds = D::new(n);
        self.i = self.es.len();
    }
    fn next(&mut self) {
        self.i -= 1;
        for &(u, v) in &self.es[self.i] {
            self.ds.unite(u, v);
        }
    }
    fn pred(&self, &(u, v): &Self::Input) -> bool {
        !self.ds.equiv(u, v)
    }
    fn count(&self) -> usize {
        self.es.len()
    }
}
