use std::collections::VecDeque;

pub fn from_root<T: Clone>(
    n: usize,
    es: &[(usize, usize, T)],
    r: usize,
) -> Vec<Vec<(usize, T)>> {
    let mut g = vec![vec![]; n];
    for &(u, v, ref w) in es {
        g[u].push((v, w.clone()));
        g[v].push((u, w.clone()));
    }

    let mut res = vec![vec![]; n];
    let mut q = VecDeque::new();
    let mut seen = vec![false; n];
    q.push_back(r);
    seen[r] = true;
    while let Some(v) = q.pop_front() {
        for (nv, nw) in g[v].drain(..) {
            if seen[nv] {
                continue;
            }
            seen[nv] = true;
            res[v].push((nv, nw));
            q.push_back(nv);
        }
    }
    res
}
