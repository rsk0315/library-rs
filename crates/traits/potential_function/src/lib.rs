//! ポテンシャル関数。

pub use binop::{CommutativeGroup, Magma};

/// ポテンシャル関数。
pub trait PotentialFunction {
    /// 要素の型。
    type Item: CommutativeGroup;

    /// 要素数 $n$ の集合 $\\{0, 1, \\dots, n-1\\}$ で初期化する。
    fn new(n: usize, cgroup: Self::Item) -> Self;
    /// 集合の要素数 $n$ を返す。
    fn len(&self) -> usize;
    /// 集合が空であれば `true` を返す。
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// ポテンシャルの差を定義する。
    ///
    /// $\\phi(x\_u)-\\phi(x\_v) = w$ とする。
    ///
    /// 呼び出し前の定義と矛盾しない場合、呼び出し前に $\\phi(x\_u)-\\phi(x\_v)$ が未定義なら
    /// `Ok(true)` を、そうでなければ `Ok(false)` を返す。
    /// 矛盾する場合、定義は変化せずに `Err(e)` を返す。ただし、`e`
    /// は呼び出し前の $\\phi(x\_u) - \\phi(x\_v)$ を表す。
    fn relate(
        &mut self,
        u: usize,
        v: usize,
        w: <Self::Item as Magma>::Set,
    ) -> Result<bool, <Self::Item as Magma>::Set>;

    /// ポテンシャルの差を求める。
    ///
    /// $\\phi(x\_u)-\\phi(x\_v) = w$ であれば `Some(w)` を返す。
    /// 未定義ならば `None` を返す。
    fn diff(&self, u: usize, v: usize) -> Option<<Self::Item as Magma>::Set>;

    /// 代表元とのポテンシャルの差を求める。
    fn repr_diff(&self, u: usize) -> (usize, <Self::Item as Magma>::Set);
}
