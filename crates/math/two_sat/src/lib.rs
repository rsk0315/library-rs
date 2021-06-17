//! 2-SAT。

use scc_::scc;

/// 2-SAT。
///
/// $f(x\_1, \\dots, x\_n) = \\bigwedge (\\bullet \\vee \\bullet)$
/// の形の論理式を $\\top$ にするような $x\_1, \\dots, x\_n$ の割り当てがあるか判定する。
/// ただし、各々の $\\bullet$ は $x\_1, \\dots, x\_n, \\lnot x\_1, \\dots \\lnot x\_n$
/// のうちのいずれかである。
///
/// # Examples
/// ```
/// use nekolib::math::TwoSat;
///
/// let mut ts = TwoSat::new(4);
/// ts.add_clause(1, 1);
/// ts.add_clause(-2, -2);
/// ts.add_clause(3, -4);
///
/// let w = ts.witness().unwrap();
/// assert!(w[1]);
/// assert!(!w[2]);
/// assert!(w[3] || !w[4]);
///
/// ts.add_clause(-1, -1);
/// assert_eq!(ts.witness(), None);
/// ```
pub struct TwoSat {
    n: usize,
    cnf: Vec<(isize, isize)>,
}

impl TwoSat {
    /// $f(x\_1, \\dots, x\_n) = \\top$ で初期化する。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::TwoSat;
    ///
    /// let ts = TwoSat::new(1);
    /// assert!(ts.witness().is_some());
    /// ```
    pub fn new(n: usize) -> Self { Self { n, cnf: vec![] } }
    /// $f(x\_1, \\dots, x\_n) \\xleftarrow{\\wedge} (x\_i \\vee x\_j)$ で更新する。
    ///
    /// ただし、$i \\lt 0$ のとき $x\_i$ は $\\lnot x\_{-i}$ を指すとする。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::TwoSat;
    ///
    /// let mut ts = TwoSat::new(3);
    /// ts.add_clause(-1, 2);
    /// ts.add_clause(1, 3);
    /// ```
    pub fn add_clause(&mut self, i: isize, j: isize) { self.cnf.push((i, j)); }
    /// 充足可能性を判定し、可能なら解を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::TwoSat;
    ///
    /// let mut ts = TwoSat::new(3);
    /// ts.add_clause(-1, 2);
    /// ts.add_clause(-2, 3);
    /// ts.add_clause(-3, -2);
    /// let w = ts.witness().unwrap();
    /// assert!(!w[1] || w[2]);
    /// assert!(!w[2] || w[3]);
    /// assert!(!w[3] || !w[2]);
    /// // w == [_, false, false, true], for example.
    ///
    /// ts.add_clause(2, 1);
    /// assert_eq!(ts.witness(), None);
    pub fn witness(&self) -> Option<Vec<bool>> {
        let n = self.n;
        let g = {
            let mut g = vec![vec![]; 2 * (n + 1)];
            for &(i, j) in &self.cnf {
                let (i, not_i) = Self::enc(i);
                let (j, not_j) = Self::enc(j);
                g[not_i].push(j);
                g[not_j].push(i);
            }
            g
        };
        let index = |&v: &usize| v;
        let delta = |&v: &usize| g[v].iter().cloned();
        let comp_id = scc(g.len(), 2..g.len(), index, delta);
        if (1..=n).any(|i| comp_id[2 * i] == comp_id[2 * i + 1]) {
            return None;
        }
        Some(
            std::iter::once(true)
                .chain((1..=n).map(|i| comp_id[2 * i + 1] < comp_id[2 * i]))
                .collect(),
        )
    }
    fn enc(i: isize) -> (usize, usize) {
        let ii = 2 * i.abs() as usize;
        if i < 0 {
            (ii + 1, ii)
        } else {
            (ii, ii + 1)
        }
    }
}
