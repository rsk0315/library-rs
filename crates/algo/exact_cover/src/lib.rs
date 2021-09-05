//! Exact cover。

/// Exact cover。
///
/// 0/1 行列 $A$ が与えられたとき、行 $\\{0, 1, \\dots, n-1\\}$ の部分集合であって、
/// 各列 $0\\le j\\lt m$ に対して $A\_{i, j}=1$ なる $i\\in S$ がちょうど 1
/// つ存在するものを探す。
///
/// $$ A = \\begin{pmatrix}
/// 0 & 0 & 1 & 0 & 1 & 1 & 0 \\\\
/// 1 & 0 & 0 & 1 & 0 & 0 & 1 \\\\
/// 0 & 1 & 1 & 0 & 0 & 1 & 0 \\\\
/// 1 & 0 & 0 & 1 & 0 & 0 & 0 \\\\
/// 0 & 1 & 0 & 0 & 0 & 0 & 1 \\\\
/// 0 & 0 & 0 & 1 & 1 & 0 & 1 \\\\
/// \\end{pmatrix} $$
/// のような行列 $A$ に対しては、$\\{0, 3, 4\\}$ が解となる。
///
/// # Idea
/// Dancing links と呼ばれるデータ構造を用いる。
///
/// `todo!()`
///
/// # Examples
/// ```
/// use nekolib::algo::ExactCover;
/// let a = vec![                 
///     vec![0, 0, 1, 0, 1, 1, 0],
///     vec![1, 0, 0, 1, 0, 0, 1],
///     vec![0, 1, 1, 0, 0, 1, 0],
///     vec![1, 0, 0, 1, 0, 0, 0],
///     vec![0, 1, 0, 0, 0, 0, 1],
///     vec![0, 0, 0, 1, 1, 0, 1],
/// ];
/// let ec = ExactCover::from_matrix(&a);
/// assert_eq!(ec.any(), Some(vec![3, 0, 4]));
/// ```
///
/// # References
/// - <https://www-cs-faculty.stanford.edu/~knuth/papers/dancing-color.ps.gz>
#[derive(Default)]
pub struct ExactCover {
    link: Vec<Node>,
    size: Vec<usize>,
    row: Vec<usize>,
    col: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Default)]
struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl ExactCover {
    /// 与えられた行列に対して前計算を行う。
    pub fn from_matrix(a: &Vec<Vec<usize>>) -> Self {
        let h = a.len();
        if h == 0 {
            return Self::default();
        }
        let w = a[0].len();
        let mut index = vec![vec![0; w]; h];
        let mut size = vec![0; w];
        let mut row = vec![0; w + 1];
        let mut col = vec![0; w + 1];
        let mut cur = w;
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 0 {
                    continue;
                }
                cur += 1;
                index[i][j] = cur;
                size[j] += 1;
                row.push(i);
                col.push(j);
            }
        }

        let mut link = vec![Node::default(); cur + 1];
        link[0].right = 1;
        link[0].left = w;
        link[w].right = 0;
        link[w].left = w - 1;
        for j in 1..w {
            link[j].right = j + 1;
            link[j].left = j - 1;
        }
        for i in 0..h {
            let first = match (0..w).find(|&j| index[i][j] != 0) {
                Some(s) => s,
                None => continue,
            };
            let mut j = first;
            loop {
                match (j + 1..w).find(|&nj| index[i][nj] != 0) {
                    Some(nj) => {
                        link[index[i][j]].right = index[i][nj];
                        link[index[i][nj]].left = index[i][j];
                        j = nj;
                    }
                    None => {
                        link[index[i][first]].left = index[i][j];
                        link[index[i][j]].right = index[i][first];
                        break;
                    }
                };
            }
        }

        for j in 0..w {
            let first = match (0..h).find(|&i| index[i][j] != 0) {
                Some(s) => s,
                None => continue,
            };
            let mut i = first;
            link[j + 1].down = index[i][j];
            link[index[i][j]].up = j + 1;
            loop {
                match (i + 1..h).find(|&ni| index[ni][j] != 0) {
                    Some(ni) => {
                        link[index[i][j]].down = index[ni][j];
                        link[index[ni][j]].up = index[i][j];
                        i = ni;
                    }
                    None => {
                        link[j + 1].up = index[i][j];
                        link[index[i][j]].down = j + 1;
                        break;
                    }
                }
            }
        }

        Self { link, size, row, col }
    }

    /// 解を全て探す。
    pub fn all(mut self) -> Vec<Vec<usize>> {
        if self.link.is_empty() || self.size.iter().any(|&x| x == 0) {
            return vec![];
        }

        let mut res = vec![];
        let mut cur = vec![];
        self.dfs(&mut cur, &mut res, false);
        res
    }

    /// 解を探す。一つ見つかった時点で打ち切る。
    pub fn any(mut self) -> Option<Vec<usize>> {
        if self.link.is_empty() || self.size.iter().any(|&x| x == 0) {
            return None;
        }

        let mut res = vec![];
        let mut cur = vec![];
        self.dfs(&mut cur, &mut res, true);
        res.pop()
    }

    fn dfs(
        &mut self,
        cur: &mut Vec<usize>,
        res: &mut Vec<Vec<usize>>,
        any: bool,
    ) {
        if self.link[0].right == 0 {
            res.push(cur.clone());
            return;
        }

        let c = self.choose();
        self.cover(c);
        let mut r = self.link[c].down;
        while r != c {
            cur.push(self.row[r]);
            let mut j = self.link[r].right;
            while j != r {
                self.cover(self.col[j] + 1);
                j = self.link[j].right;
            }
            self.dfs(cur, res, any);
            if any && !res.is_empty() {
                return;
            }

            cur.pop();
            let mut j = self.link[r].left;
            while j != r {
                self.uncover(self.col[j] + 1);
                j = self.link[j].left;
            }
            r = self.link[r].down;
        }

        self.uncover(c);
    }

    fn choose(&self) -> usize {
        let mut j = self.link[0].right;
        let mut c = j;
        let mut s = self.size[j - 1];
        while j != 0 {
            if self.size[j - 1] < s {
                c = j;
                s = self.size[j - 1];
            }
            j = self.link[j].right;
        }
        c
    }

    fn cover(&mut self, c: usize) {
        let left = self.link[c].left;
        let right = self.link[c].right;
        self.link[right].left = left;
        self.link[left].right = right;
        let mut i = self.link[c].down;
        while i != c {
            let mut j = self.link[i].right;
            while j != i {
                let up = self.link[j].up;
                let down = self.link[j].down;
                self.link[down].up = up;
                self.link[up].down = down;
                self.size[self.col[j]] -= 1;
                j = self.link[j].right;
            }
            i = self.link[i].down;
        }
    }

    fn uncover(&mut self, c: usize) {
        let mut i = self.link[c].up;
        while i != c {
            let mut j = self.link[i].left;
            while j != i {
                self.size[self.col[j]] += 1;
                let down = self.link[j].down;
                let up = self.link[j].up;
                self.link[down].up = j;
                self.link[up].down = j;
                j = self.link[j].left;
            }
            i = self.link[i].up;
        }
        let right = self.link[c].right;
        let left = self.link[c].left;
        self.link[right].left = c;
        self.link[left].right = c;
    }
}
