//! functional graph。

use std::fmt::Debug;

/// function graph。
///
/// # Todo
/// 辺にモノイドを持たせて n 個進んだときの fold をするのは
/// ダブリングテーブルとかが必要になって、周期検出をしたいだけのときには
/// オーバーヘッドが大きそう。
///
/// なので、`FoldableFunctionalGraph` とかを別に作ってみる。
/// それの内部でこれを持っても使えそう。
///
/// # Complexity
/// $O(n)$ time.
///
/// # Examples
/// ```text
/// +---> 1 ----+     +--- 6 <--- 5
/// |           |     |
/// |           v     |
/// |           2 <---+--- 7     12
/// 0           |
/// ^           v                 8
/// |           3 <------ 11      |
/// |           |                 v
/// +---- 4 <---+     10 <------> 9
/// ```
/// |$i$|0|1|2|3|4|5|6|7|8|9|10|11|12|
/// |---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
/// |$f(i)$|1|2|3|4|0|6|2|2|9|10|9|3|12|
/// |$\\mu\_i$|0|0|0|0|0|2|1|1|1|0|0|1|0|
/// |$\\lambda\_i$|5|5|5|5|5|5|5|5|2|2|2|5|1|
/// ```
/// use nekolib::graph::FunctionalGraph;
///
/// let f = vec![1, 2, 3, 4, 0, 6, 2, 2, 9, 10, 9, 3, 12];
/// let n = f.len();
/// let g: FunctionalGraph = f.into();
/// let mu = vec![0, 0, 0, 0, 0, 2, 1, 1, 1, 0, 0, 1, 0];
/// let lambda = vec![5, 5, 5, 5, 5, 5, 5, 5, 2, 2, 2, 5, 1];
/// for i in 0..n {
///     assert_eq!(g.mu_lambda(i), (mu[i], lambda[i]));
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionalGraph {
    to: Vec<usize>,
    mu_lambda: Vec<(usize, usize)>,
}

impl From<Vec<usize>> for FunctionalGraph {
    fn from(f: Vec<usize>) -> Self {
        let n = f.len();
        let none = (n, 0);
        let mut mu_lambda = vec![none; n];
        let mut stack = vec![];
        let mut index = vec![n; n];
        for mut i in 0..n {
            while mu_lambda[i] == none {
                if index[i] < n {
                    break;
                }
                index[i] = stack.len();
                stack.push(i);
                i = f[i];
            }
            if mu_lambda[i] == none {
                let lambda = stack.len() - index[i];
                let seen = i;
                while let Some(j) = stack.pop() {
                    mu_lambda[j] = (0, lambda);
                    if j == seen {
                        break;
                    }
                }
            }
            while let Some(j) = stack.pop() {
                mu_lambda[j] = mu_lambda[f[j]];
                mu_lambda[j].0 += 1;
            }
        }
        Self { to: f, mu_lambda }
    }
}

impl FunctionalGraph {
    /// $(\\mu\_i, \\lambda\_i)$ を返す。
    pub fn mu_lambda(&self, i: usize) -> (usize, usize) { self.mu_lambda[i] }
}
