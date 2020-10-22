//! disjoint sparse table。

use super::super::traits::binop;
use super::super::traits::fold;
use super::super::utils::buf_range;

use std::convert::From;
use std::ops::{Index, Range, RangeBounds};

use binop::Monoid;
use buf_range::bounds_within;
use fold::Fold;

/// disjoint sparse table。
///
/// 要素数 $n$ の配列の任意の区間について、モノイド積の値を計算できる。
/// 値の更新はできない。
/// 半群を返すことにしてもよいが、要検討。
///
/// # Complexity
///
/// 前処理の際、モノイド積を高々 $n\\lfloor\\log\_2(n)-1\\rfloor$ 回計算するが、
/// がんばって重複を削減することでもう少し削減でき、次の値で上から抑えられる。
/// $$ \\begin{aligned}
/// n\\cdot\\lceil{\\log\_2(n)-3}\\rceil + \\lceil{\\log\_2(n)}\\rceil + 2
/// + \\begin{cases}
/// \\log\_2(n), & \\text{if }n\\text{ is power of }2; \\\\
/// 2\^{\\lfloor\\log\_2(n)\\rfloor}, & \\text{otherwise.}
/// \\end{cases}
/// \\end{aligned} $$
///
/// また、クエリ処理の際、高々 $1$ 回（！）のモノイド積を計算する。
///
/// モノイド積の計算コストが非常に高いときは前処理の削減は有用だと思うが、
/// そうでないときにどうなるかは要実測。
///
/// 現時点では、使い回す箇所の実装方法はよくわかっていない（たぶんできると思う）。
///
/// ## Precise Analysis
///
/// 実際の回数は以下のコードで計算できる（はず）。
/// ```
/// /// 要素数 `n` での前処理における計算回数を返す。
/// fn count(n: usize) -> usize {
///     if n <= 2 {
///         return 0;
///     }
///     g(n - 1)
///         + if n.is_power_of_two() {
///             n.trailing_zeros() as usize
///         } else {
///             n.next_power_of_two() / 2
///         }
///         - 1
/// }
///
/// assert_eq!(count(3), 1);
/// assert_eq!(count(10), 14);
/// assert_eq!(count(1000), 7008);
/// assert_eq!(count(1_000_000), 16_980_635);
///
/// /// 各段における寄与分の和を返す。
/// fn g(n: usize) -> usize {
///     (0..)
///         .take_while(|&k| n >= 2_usize.pow(k + 1))
///         .map(|k| f(k, n - 2_usize.pow(k + 1)))
///         .sum::<usize>()
/// }
///
/// /// k 段目における寄与分を返す。
/// fn f(k: u32, n: usize) -> usize {
///     let p = 2_usize.pow(k);
///     n / (2 * p) * p
///         + if n / p % 2 == 1 { n % p + 1 } else { 0 }
///         + (n + 1) / (2 * p) * (p - 1)
/// }
/// ```
pub struct DisjointSparseTable<M: Monoid> {
    len: usize,
    height: usize,
    buf: Vec<Vec<M::Set>>,
}

impl<M, B> Fold<B> for DisjointSparseTable<M>
where
    M: Monoid,
    M::Set: Clone,
    B: RangeBounds<usize>,
{
    type Output = M;
    fn fold(&self, b: B) -> M::Set {
        let Range { start, end } = bounds_within(b, self.len);
        todo!();
    }
}

impl<M> From<Vec<M::Set>> for DisjointSparseTable<M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn from(v: Vec<M::Set>) -> Self {
        todo!();
    }
}

impl<M> Index<usize> for DisjointSparseTable<M>
where
    M: Monoid,
    M::Set: Clone,
{
    type Output = M::Set;
    fn index(&self, i: usize) -> &Self::Output {
        &self.buf[0][i]
    }
}
