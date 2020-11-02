//! 最短距離 (Dijkstra)。

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Dijkstra 法に基づく最短距離。
///
/// # Parameters
/// 後でちゃんと書きます。
///
/// [これ](https://niuez.github.io/posts/impl_abstract_dijkstra/)
/// をリスペクトしているつもり。
///
/// # Complexity
/// 二分ヒープを用いる実装なので、$O(|E|\\log(|V|))$ 時間。
pub fn dijkstra<V, W, I, D>(
    n: usize,
    s: V,
    zero: W,
    index: I,
    delta: D,
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
