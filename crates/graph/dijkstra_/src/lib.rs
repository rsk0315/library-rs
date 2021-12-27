//! 最短距離 (Dijkstra)。

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Dijkstra 法に基づく最短距離。
///
/// # Parameters
/// - `n`: 頂点数
/// - `s`: 始点
/// - `zero`: 距離を表す型の $0$
/// - `index`: 頂点から添字への番号づけをする関数
/// - `delta`: 頂点 `v` と関数 `search` を受け取る関数
///
/// `delta` は、`v` の各隣接頂点 `nv` とそこへの距離 `ew` に対して、
/// `search(nv, ew)` を呼び出す必要がある。
///
/// # Examples
/// `g[v]` が `v` の隣接頂点を持つ、よくある隣接リストにおける例を載せる。
/// 次のようなグラフである。
///
/// ```text
///      2        3
/// (0) ---> (1) ---> (2)
///  ^        |        |
///  | 1      |        | 4
///  |        |  9     v
/// (3)       +-----> (4)
/// ```
///
/// ```
/// use nekolib::graph::dijkstra;
///
/// let g = vec![
///     vec![(1, 2)],
///     vec![(2, 3), (4, 9)],
///     vec![(4, 4)],
///     vec![(0, 1)],
///     vec![],
/// ];
/// let index = |&v: &usize| v;
/// let delta = |&v: &usize| g[v].iter().cloned();
/// let dist = dijkstra(5, 0, 0_i32, index, delta);
///
/// assert_eq!(dist, vec![Some(0), Some(2), Some(5), None, Some(9)]);
/// ```
///
/// 頂点数の上限 $n$ さえ既知であれば、`index` は動的に決められる。
/// ```
/// use std::collections::BTreeMap;
///
/// use nekolib::graph::dijkstra;
///
/// let n = 10;
///
/// let s = 10001;
/// let t = 10015;
/// let u = t - 1;
/// let mut enc = BTreeMap::new();
/// let index = |&v: &usize| match enc.get(&v) {
///     Some(&i) => i,
///     None => {
///         let len = enc.len();
///         enc.insert(v, len);
///         len
///     }
/// };
/// let delta =
///   |&v: &usize| Some((v + 2, 1)).filter(|&(v, _)| v <= t).into_iter();
///
/// let res = dijkstra(n, s, 0, index, delta);
/// assert_eq!(res[enc[&t]], Some(7));
/// assert!(!enc.contains_key(&u))
/// ```
///
/// # Notes
/// 複数のグラフを扱う際に `delta` を使い回すと、意図しないグラフを見てしまいがちなので注意。
///
/// # Complexity
/// 二分ヒープを用いる実装なので、$O(|E|\\log(|V|))$ 時間。
/// ここで、$V$ は頂点集合、$E$ は辺集合である。
///
/// # References
/// - <https://niuez.github.io/posts/impl_abstract_dijkstra/>
///     - これを意識していますが、辺の取得にイテレータを使っているので少々異なります。
pub fn dijkstra<V, W, E>(
    n: usize,
    s: V,
    zero: W,
    mut index: impl FnMut(&V) -> usize,
    mut delta: impl FnMut(&V) -> E,
) -> Vec<Option<W>>
where
    V: Ord,
    W: Ord + std::ops::Add<W, Output = W> + Clone,
    E: Iterator<Item = (V, W)>,
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
        for (nv, ew) in delta(&v) {
            let nw = w.clone() + ew;
            match &dist[index(&nv)] {
                Some(cw) if cw <= &nw => {}
                _ => {
                    dist[index(&nv)] = Some(nw.clone());
                    pq.push(Reverse((nw, nv)));
                }
            }
        }
    }
    dist
}
