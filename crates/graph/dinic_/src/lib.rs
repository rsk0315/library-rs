//! 最大流 (Dinic)。

use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::ops::{AddAssign, SubAssign};
use std::rc::Rc;

/// Dinic 法に基づく最大流。
///
/// # Idea
/// `todo!()`
///
/// # Complexity
/// $O(|V|^2|E|)$ 時間。
///
/// 辺容量が整数のとき、多くのパラメータによって bound できることが知られている。
/// 以下の条件は、高々定数個の例外があってもよい。
///
/// - 最大流を $F$ として $O(F|E|)$ 時間。
/// - 辺容量が高々 $k$ のとき $O(k\\,|E|^{3/2})$ 時間。
/// - 辺容量が高々 $k$ で多重辺がないとき $O(k\\,|V|^{2/3}|E|)$ 時間。
/// - 各頂点を通れるフロー量が高々 $k$ のとき $O(k\\,|V|^{1/2}|E|)$ 時間。
///     - $k \\ge \\max\_v \\min\\{\\sum\_{e\\in\\delta^+(v)} u\_e, \\sum\_{e\\in\\delta^-(v)} u\_e\\}$.
///     - 二部マッチングであれば $k = 1$ であり、$O(|V|^{1/2}|E|)$ 時間。
///
/// # Examples
///
/// 次のようなグラフを考える。
/// [Wikipedia](https://en.wikipedia.org/wiki/Dinic%27s_algorithm#Example) にある例である。
///
/// ```text
///       10           4       10
///  +--------> [1] ----> [3] -------+
///  |           | \       ^ 4       |
///  |           |  \  8   |         v
/// [0]        2 |   \--> [4] ----> [5]
///  |           |         ^   10
///  |    10     v     9   |
///  +--------> [2] -------+
/// ```
///
/// 流れるフローは次の通りで、$19$ である。
///
/// - $(0, 1, 3, 5)$ に $4$
/// - $(0, 1, 4, 5)$ に $6$
/// - $(0, 2, 4, 5)$ に $4$
/// - $(0, 2, 4, 3, 5)$ に $5$
///
/// ```
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// use nekolib::graph::dinic;
///
/// let es = vec![
///     vec![(1, 10), (2, 10)],       // 0
///     vec![(2, 2), (3, 4), (4, 8)], // 1
///     vec![(4, 9)],                 // 2
///     vec![(5, 10)],                // 3
///     vec![(3, 6), (5, 10)],        // 4
///     vec![],                       // 5
/// ];
/// let n = es.len();
/// let g = {
///     let mut g = vec![vec![]; 6]; // [from]: [(to, capacity, rev), ...]
///     for from in 0..n {
///         for &(to, capacity) in &es[from] {
///             let from_len = g[from].len();
///             let to_len = g[to].len();
///             g[from].push((to, Rc::new(RefCell::new(capacity)), to_len));
///             g[to].push((from, Rc::new(RefCell::new(0)), from_len));
///         }
///     }
///     g
/// };
///
/// let index = |&v: &usize| v;
/// let delta = |&v: &usize| g[v].iter().map(|&(nv, ref w, r)| (nv, w.clone(), r));
/// let rev = |&nv: &usize, &r: &usize| g[nv][r].1.clone();
///
/// let s = 0;
/// let t = n - 1;
/// let max_flow = dinic(n, s, t, 0..n, 0, index, delta, rev);
/// assert_eq!(max_flow, 19);
/// ```
///
/// # References
/// - <https://misawa.github.io/others/flow/dinic_time_complexity.html>
pub fn dinic<V, W, R, F>(
    n: usize,
    s: V,
    t: V,
    vs: impl Iterator<Item = V> + Clone,
    zero: W,
    index: impl Fn(&V) -> usize + Copy,
    delta: impl Fn(&V) -> F + Copy,
    rev: impl Fn(&V, &R) -> Rc<RefCell<W>> + Copy,
) -> W
where
    V: Clone,
    W: Ord + Clone + AddAssign + SubAssign,
    R: Clone,
    F: Iterator<Item = (V, Rc<RefCell<W>>, R)>,
{
    let mut res = zero.clone();
    loop {
        let level = dual(n, s.clone(), zero.clone(), index, delta);
        if level[index(&t)] == n {
            break;
        }
        let iter: Vec<_> = vs
            .clone()
            .map(|v| Rc::new(RefCell::new(delta(&v).peekable())))
            .collect();
        loop {
            match primal(&s, &t, zero.clone(), &level, index, rev, &iter) {
                Some(f) => res += f,
                None => break,
            }
        }
    }
    res
}

fn dual<V, W, R, F>(
    n: usize,
    s: V,
    zero: W,
    index: impl Fn(&V) -> usize,
    delta: impl Fn(&V) -> F,
) -> Vec<usize>
where
    V: Clone,
    W: Ord + Clone + AddAssign + SubAssign,
    R: Clone,
    F: Iterator<Item = (V, Rc<RefCell<W>>, R)>,
{
    let mut level = vec![n; n];
    let mut q = VecDeque::new();
    level[index(&s)] = 0;
    q.push_back(s);
    while let Some(v) = q.pop_front() {
        let i = index(&v);
        for (nv, w, _) in delta(&v) {
            let ni = index(&nv);
            if *w.borrow() > zero && level[ni] == n {
                level[ni] = level[i] + 1;
                q.push_back(nv);
            }
        }
    }
    level
}

fn primal<V, W, R, I>(
    s: &V,
    t: &V,
    zero: W,
    level: &[usize],
    index: impl Fn(&V) -> usize,
    rev: impl Fn(&V, &R) -> Rc<RefCell<W>>,
    iter: &[Rc<RefCell<Peekable<I>>>],
) -> Option<W>
where
    V: Clone,
    W: Ord + Clone + AddAssign + SubAssign,
    R: Clone,
    I: Iterator<Item = (V, Rc<RefCell<W>>, R)>,
{
    let ti = index(t);
    let mut es = vec![];
    let mut vis = vec![index(s)];
    'find_path: while let Some(&vi) = vis.last() {
        if vi == ti {
            break;
        }
        loop {
            if let Some((nv, w, r)) = iter[vi].borrow_mut().peek() {
                let nvi = index(&nv);
                if *w.borrow() > zero && level[vi] < level[nvi] {
                    es.push((w.clone(), nv.clone(), r.clone()));
                    vis.push(nvi);
                    continue 'find_path;
                }
            } else {
                break;
            }

            iter[vi].borrow_mut().next();
        }
        es.pop();
        vis.pop();
        if let Some(&vi) = vis.last() {
            iter[vi].borrow_mut().next();
        }
    }

    if es.is_empty() {
        return None;
    }

    let f = es.iter().map(|(e, _, _)| e.borrow().clone()).min().unwrap();
    eprintln!("{:?}", vis);

    for (w, nv, r) in es {
        *w.borrow_mut() -= f.clone();
        *rev(&nv, &r).borrow_mut() += f.clone();
    }

    Some(f)
}

#[test]
fn dinic_misawa_hack() {
    // https://gist.github.com/MiSawa/47b1d99c372daffb6891662db1a2b686
    let n = 500;
    let (s, a, b, c, t) = (0, 1, 2, 3, 4);
    let mut uv = (5, 6);
    let mut es = vec![
        (s, a, 1),
        (s, b, 2),
        (b, a, 2),
        (c, t, 2),
        (a, uv.0, 3),
        (a, uv.1, 3),
    ];
    while uv.1 + 2 < n {
        let nuv = (uv.0 + 2, uv.1 + 2);
        for &x in &[uv.0, uv.1] {
            for &y in &[nuv.0, nuv.1] {
                es.push((x, y, 3));
            }
        }
        uv = nuv;
    }
    es.push((uv.0, c, 3));
    es.push((uv.1, c, 3));

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

    let index = |&v: &usize| v;
    let delta =
        |&v: &usize| g[v].iter().map(|&(nv, ref w, r)| (nv, w.clone(), r));
    let rev = |&nv: &usize, &r: &usize| g[nv][r].1.clone();
    let flow = dinic(n, s, t, 0..n, 0, index, delta, rev);

    assert_eq!(flow, 2);
}
