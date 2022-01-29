//! 素集合に関するトレイトです。

/// 共通要素を持たない集合族で、併合が可能なもの。
pub trait DisjointSet {
    /// 集合族を $\\{\\{0\\}, \\{1\\}, \\dots, \\{n-1\\}\\}$ で初期化する。
    fn new(n: usize) -> Self;
    /// 集合族全体に含まれる要素数 $n$ を返す。
    fn len(&self) -> usize;
    /// 集合族が空であれば `true` を返す。
    fn is_empty(&self) -> bool { self.len() == 0 }
    /// $u$ を含む集合と $v$ を含む集合を併合する。
    /// 集合族に変化があれば `true` を返す。
    /// $u$ と $v$ が元々同じ集合に含まれていれば `false` を返す。
    fn unite(&mut self, u: usize, v: usize) -> bool;
    /// $u$ を含む集合の代表元を返す。
    fn repr(&self, u: usize) -> usize;
    /// $u$ を含む集合の要素数を返す。
    fn count(&self, u: usize) -> usize;
    /// $u$ と $v$ が同じ集合に含まれていれば `true` を返す。
    fn equiv(&self, u: usize, v: usize) -> bool { self.repr(u) == self.repr(v) }
    /// $u$ を含む集合の要素を列挙する。
    fn subset(&self, u: usize) -> Vec<usize> {
        (0..self.len()).filter(|&v| self.equiv(u, v)).collect()
    }
    /// 分割を返す。
    ///
    /// $u$ が代表元のとき、$u$ 番目の `Vec` にそれと等価な要素たちが入る。
    fn partition(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for i in 0..self.len() {
            res[self.repr(i)].push(i);
        }
        res
    }
}
