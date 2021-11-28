//! 木。

use std::collections::VecDeque;

/// 木。
pub struct Tree<T> {
    bfs: Vec<usize>,
    par: Vec<usize>,
    rank: Vec<usize>,
    start: Vec<usize>,
    es: Vec<(usize, T)>,
}

impl<T> From<Vec<Vec<(usize, T)>>> for Tree<T> {
    fn from(g: Vec<Vec<(usize, T)>>) -> Self {
        let n = g.len();
        let (bfs, par, rank) = Self::bfs(&g);

        let start = {
            let mut a = vec![0; n + 1];
            for i in 0..n {
                a[i + 1] = a[i] + g[i].len();
            }
            a
        };

        let es: Vec<_> = g.into_iter().flatten().collect();
        Self { bfs, par, rank, start, es }
    }
}

impl<T> Tree<T> {
    fn bfs(g: &Vec<Vec<(usize, T)>>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
        let n = g.len();
        if n == 0 {
            return (vec![], vec![], vec![]);
        }

        let mut q = VecDeque::new();
        q.push_back(0);
        let mut par = vec![n; n];
        let mut bfs = vec![0];
        let mut rank = vec![0; n];
        par[0] = g[0].len();
        while let Some(v) = q.pop_front() {
            for (i, nv) in g[v].iter().map(|(nv, _)| *nv).enumerate() {
                if par[nv] < n {
                    par[v] = i;
                    continue;
                }
                rank[nv] = i;
                q.push_back(nv);
                bfs.push(nv);
            }
        }
        (bfs, par, rank)
    }
}

use std::fmt::Debug;

impl<T: Debug> Tree<T> {
    /// 全方位木 DP。
    ///
    /// `empty` と `map` と `fold` で作られる catamorphism を考える。
    /// 頂点 $i$ ($0\\le i\\lt n$) を根としたときの catamorphism をそれぞれ求める。
    ///
    /// # Idea
    /// `todo!()`
    ///
    /// # Implementation notes
    /// `empty` は本来 2 種類必要なはずだが、共通であることが多いのでまとめている。
    /// 半群からモノイドを機械的に作るのと同様に必要に応じて対処できるはず。
    ///
    /// # Complexity
    /// $O(n)$ time.
    ///
    /// # Examples
    /// ```
    /// use nekolib::graph::Tree;
    ///
    /// let n = 6;
    /// let es = vec![(0, 1), (0, 2), (1, 3), (1, 4), (1, 5)];
    /// let g = {
    ///     let mut g = vec![vec![]; n];
    ///     for &(u, v) in &es {
    ///         g[u].push((v, ()));
    ///         g[v].push((u, ()));
    ///     }
    ///     g
    /// };
    /// let tree: Tree<_> = g.into();
    ///
    /// // max distance
    /// let empty = 0;
    /// let map = |&x: &usize, _: &()| x + 1;
    /// let fold = |&x: &usize, &y: &usize| x.max(y);
    /// assert_eq!(tree.cata(empty, map, fold), [2, 2, 3, 3, 3, 3]);
    ///
    /// // sum of distance
    /// let empty = (0, 0);
    /// let map = |&(d, c): &(usize, usize), _: &()| (d + c + 1, c + 1);
    /// let fold = |&x: &(usize, usize), &y: &(usize, usize)| (x.0 + y.0, x.1 + y.1);
    /// assert_eq!(
    ///     tree.cata(empty, map, fold).into_iter().map(|x| x.0).collect::<Vec<_>>(),
    ///     [8, 6, 12, 10, 10, 10]
    /// );
    /// ```
    pub fn cata<U: Clone + Debug>(
        &self,
        empty: U,
        mut map: impl FnMut(&U, &T) -> U,
        mut fold: impl FnMut(&U, &U) -> U,
    ) -> Vec<U> {
        let n = self.bfs.len();
        if n == 0 {
            return vec![];
        }

        let mut left = vec![empty.clone(); 2 * (n - 1) + n];
        let mut right = vec![empty.clone(); 2 * (n - 1) + n];
        for &v in self.bfs[1..].iter().rev() {
            let vl = self.start[v];
            let deg = self.start[v + 1] - vl;
            let off = vl + v;
            let p = self.par[v];
            for i in 0..p {
                left[off + i + 1] = fold(&left[off + i], &left[off + i + 1]);
            }
            for i in (p + 1..deg).rev() {
                right[off + i] = fold(&right[off + i], &right[off + i + 1]);
            }
            let cur = fold(&left[off + p], &right[off + p + 1]);
            let &(nv, ref w) = &self.es[vl + p];
            let r = self.start[nv] + nv + self.rank[v];
            left[r + 1] = map(&cur, w);
            right[r] = left[r + 1].clone();
        }

        let mut dp = vec![empty.clone(); n];

        for &v in &self.bfs {
            let vl = self.start[v];
            let deg = self.start[v + 1] - vl;
            let off = vl + v;
            let p = self.par[v];
            for i in if v > 0 { p } else { 0 }..deg {
                left[off + i + 1] = fold(&left[off + i], &left[off + i + 1]);
            }
            for i in (0..deg.min(p + 1)).rev() {
                right[off + i] = fold(&right[off + i], &right[off + i + 1]);
            }
            for i in (0..deg).filter(|&i| self.par[v] != i) {
                let &(nv, ref w) = &self.es[vl + i];
                let r = self.start[nv] + nv + self.par[nv];
                let cur = fold(&left[off + i], &right[off + i + 1]);
                left[r + 1] = map(&cur, w);
                right[r] = left[r + 1].clone();
            }
            dp[v] = right[off].clone();
        }

        dp
    }
}
