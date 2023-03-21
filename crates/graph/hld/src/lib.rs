use std::fmt::Debug;

/// <https://codeforces.com/blog/entry/53170>, Easiest HLD with subtree queries.
#[derive(Clone, Debug)]
pub struct Hld {
    perm: Vec<usize>,
    perm_inv: Vec<usize>,
    inout: Vec<(usize, usize)>,
    heavy: Vec<usize>,
    par: Vec<usize>,
    depth: Vec<usize>,
}

impl Hld {
    // g[v] は、子方向への隣接頂点のみを持つとする。
    pub fn new(mut g: Vec<Vec<usize>>, r: usize) -> Self {
        let n = g.len();
        let mut size = vec![0; n];
        Self::dfs_size(&mut g, r, &mut size);

        let perm = Self::dfs_order(&g, r);
        let perm_inv = Self::inv(&perm);
        let g = Self::relabel(&g, &perm);
        let (inout, heavy) = Self::dfs_hld(&g, r);

        let (par, depth) = Self::par_depth(&g, r);
        Self { perm, perm_inv, inout, heavy, par, depth }
    }

    pub fn lca(&self, ou: usize, ov: usize) -> usize {
        let u = self.perm[ou];
        let v = self.perm[ov];
        let w = self.lca_inner(u, v);
        self.perm_inv[w]
    }

    fn lca_inner(&self, u: usize, v: usize) -> usize {
        let dh = |v| self.depth[self.heavy[v]];
        let (mut lo, mut hi) = if dh(u) > dh(v) { (u, v) } else { (v, u) };

        while self.heavy[lo] != self.heavy[hi] {
            lo = self.par[self.heavy[lo]];
            if dh(lo) < dh(hi) {
                std::mem::swap(&mut lo, &mut hi);
            }
        }
        if self.depth[lo] > self.depth[hi] { hi } else { lo }
    }

    fn dfs_size(g: &mut [Vec<usize>], v: usize, size: &mut [usize]) {
        size[v] = 1;
        if g[v].is_empty() {
            return;
        }
        for i in 0..g[v].len() {
            let nv = g[v][i];
            Self::dfs_size(g, nv, size);
            size[v] += size[nv];
            if size[nv] > size[g[v][0]] {
                g[v].swap(0, i);
            }
        }
    }

    fn dfs_order(g: &[Vec<usize>], r: usize) -> Vec<usize> {
        fn dfs(g: &[Vec<usize>], v: usize, t: &mut usize, order: &mut [usize]) {
            order[v] = *t;
            *t += 1;
            for &nv in &g[v] {
                dfs(g, nv, t, order);
            }
        }

        let n = g.len();
        let mut t = 0;
        let mut order = vec![0; n];
        dfs(g, r, &mut t, &mut order);
        order
    }

    fn inv(p: &[usize]) -> Vec<usize> {
        let n = p.len();
        let mut q = vec![0; n];
        for i in 0..n {
            q[p[i]] = i;
        }
        q
    }

    fn relabel(g: &[Vec<usize>], p: &[usize]) -> Vec<Vec<usize>> {
        let n = g.len();
        let mut h = vec![vec![]; n];
        for v in 0..n {
            for &nv in &g[v] {
                h[p[v]].push(p[nv]);
            }
        }
        h
    }

    fn dfs_hld(
        g: &[Vec<usize>],
        r: usize,
    ) -> (Vec<(usize, usize)>, Vec<usize>) {
        fn dfs(
            g: &[Vec<usize>],
            v: usize,
            inout: &mut [(usize, usize)],
            next: &mut [usize],
            t: &mut usize,
        ) {
            inout[v].0 = *t;
            *t += 1;
            for &nv in &g[v] {
                next[nv] = if nv == g[v][0] { next[v] } else { nv };
                dfs(g, nv, inout, next, t);
            }
            inout[v].1 = *t;
            *t += 1;
        }

        let n = g.len();
        let mut inout = vec![(0, 0); n];
        let mut next = vec![0; n];
        let mut t = 0;
        dfs(g, r, &mut inout, &mut next, &mut t);
        (inout, next)
    }

    fn par_depth(g: &[Vec<usize>], r: usize) -> (Vec<usize>, Vec<usize>) {
        fn dfs(
            g: &[Vec<usize>],
            v: usize,
            par: &mut [usize],
            depth: &mut [usize],
        ) {
            for &nv in &g[v] {
                par[nv] = v;
                depth[nv] = depth[v] + 1;
                dfs(g, nv, par, depth);
            }
        }

        let n = g.len();
        let mut par = vec![0; n];
        let mut depth = vec![0; n];
        dfs(g, r, &mut par, &mut depth);
        (par, depth)
    }
}
