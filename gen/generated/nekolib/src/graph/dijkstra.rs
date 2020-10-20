use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra<V, D, I, W>(
    n: usize,
    s: V,
    zero: W,
    delta: D,
    index: I,
) -> Vec<Option<W>>
where
    D: Fn(&V, &mut dyn FnMut(V, W)),
    I: Fn(&V) -> usize,
    W: Ord + std::ops::Add<W, Output = W> + Clone,
    V: Ord + Clone,
{
    let mut dist: Vec<Option<W>> = vec![None; n];
    let si = index(&s);
    dist[si] = Some(zero);
    let mut pq: BinaryHeap<_> =
        vec![Reverse((dist[si].clone().unwrap(), s))].into();

    while let Some(Reverse((w, v))) = pq.pop() {
        match &dist[index(&v)] {
            Some(cw) if cw < &w => continue,
            _ => {}
        }
        delta(&v, &mut |nv, ew| {
            let nw = w.clone() + ew;
            match &dist[index(&nv)] {
                Some(cw) if cw <= &nw => {}
                _ => {
                    dist[index(&nv)] = Some(nw.clone());
                    pq.push(Reverse((nw, nv)));
                }
            }
        });
    }
    dist
}
