use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use sssp::Sssp;

pub struct DijkstraSssp<V, W, I> {
    cost: Vec<Option<W>>,
    prev: Vec<Option<V>>,
    index: I,
    src: V,
}

impl<V, W, I> DijkstraSssp<V, W, I> {
    pub fn new<D, J>(
        src: V,
        len: usize,
        zero: W,
        index: I,
        mut delta: D,
    ) -> Self
    where
        V: Ord + Clone,
        W: Add<Output = W> + Ord + Clone,
        I: Fn(&V) -> usize,
        D: FnMut(&V) -> J,
        J: Iterator<Item = (V, W)>,
    {
        let mut cost: Vec<_> = (0..len).map(|_| None).collect();
        let mut prev: Vec<_> = (0..len).map(|_| None).collect();
        let mut heap = BinaryHeap::new();
        cost[index(&src)] = Some(zero.clone());
        heap.push((Reverse(zero), src.clone()));
        while let Some((Reverse(w), v)) = heap.pop() {
            if let Some(cur_w) = &cost[index(&v)] {
                if cur_w > &w {
                    continue;
                }
            }
            for (nv, dw) in delta(&v) {
                let nw = w.clone() + dw;
                let ni = index(&nv);
                match &cost[ni] {
                    Some(cur_w) if cur_w <= &nw => {}
                    _ => {
                        cost[ni] = Some(nw.clone());
                        prev[ni] = Some(v.clone());
                        heap.push((Reverse(nw), nv));
                    }
                }
            }
        }

        Self { src, cost, prev, index }
    }
}

impl<V, W, I> Sssp<V> for DijkstraSssp<V, W, I>
where
    V: Eq + Clone,
    W: Clone,
    I: Fn(&V) -> usize,
{
    type Cost = W;
    type Path = std::vec::IntoIter<V>;
    fn cost(&self, dst: &V) -> Option<W> {
        self.cost[(self.index)(dst)].clone()
    }
    fn path(&self, dst: &V) -> Self::Path {
        let mut i = (self.index)(dst);
        let mut res = vec![];
        if self.prev[i].is_none() {
            if &self.src == dst {
                res.push(dst.clone());
            }
            return res.into_iter();
        }
        res.push(dst.clone());
        while let Some(v) = &self.prev[i] {
            i = (self.index)(v);
            res.push(v.clone());
        }
        res.reverse();
        res.into_iter()
    }
}

#[test]
fn sanity_check() {
    let g = vec![vec![(1, 10), (2, 15)], vec![(2, 10)], vec![], vec![]];
    let index = |&v: &usize| v;
    let delta = |&v: &usize| g[v].iter().copied();

    let sssp = DijkstraSssp::new(0, 4, 0, index, delta);
    assert_eq!(sssp.cost(&0), Some(0));
    assert_eq!(sssp.cost(&1), Some(10));
    assert_eq!(sssp.cost(&2), Some(15));
    assert_eq!(sssp.cost(&3), None);

    assert_eq!(sssp.path(&0).collect::<Vec<_>>(), [0]);
    assert_eq!(sssp.path(&1).collect::<Vec<_>>(), [0, 1]);
    assert_eq!(sssp.path(&2).collect::<Vec<_>>(), [0, 2]);
    assert_eq!(sssp.path(&3).collect::<Vec<_>>(), []);
}
