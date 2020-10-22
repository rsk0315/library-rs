//! disjoint sparse table。

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
/// 前処理の際、モノイド積を高々 $n\\cdot\\lfloor\\log\_2(n)-1\\rfloor$ 回計算するが、
/// がんばって重複を削減することでもう少し削減でき、次の値で上から抑えられる。
/// $$ \\begin{aligned}
/// n\\cdot\\lceil{\\log\_2(n)-3}\\rceil + 2\\cdot\\lceil{\\log\_2(n)}\\rceil + 2
/// \\end{aligned} $$
///
/// また、クエリ処理の際、高々 $1$ 回（！）のモノイド積を計算する。
/// 与えられた区間が前処理で計算した区間であるか、長さが $1$ 以下の場合は、
/// 新たにモノイド積を計算せずに答えを返す。
/// そうでない場合はちょうど $1$ 回のモノイド積を計算する。
///
///
/// モノイド積の計算コストが非常に高いときは削減は有用だと思うが、
/// そうでないときにどうなるかは要実測。
/// 削減のための計算コストが無視できないかもしれないので。
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
///
/// # Examples
/// ```
/// use nekolib::ds::DisjointSparseTable;
/// use nekolib::traits::Fold;
/// use nekolib::utils::OpAdd;
///
/// let dst: DisjointSparseTable<OpAdd<i32>> = vec![1, 6, 3, 8, 4].into();
/// assert_eq!(dst.fold(1..=3), 17);
/// assert_eq!(dst.fold(..), 22);
/// ```
pub struct DisjointSparseTable<M: Monoid> {
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
        let Range { start, end } = bounds_within(b, self.buf[0].len());
        if start >= end {
            return M::id();
        }
        let len = end - start;
        let end = end - 1;
        if start == end {
            return self.buf[0][start].clone();
        }
        let row = ((start ^ end) + 1).next_power_of_two().trailing_zeros() - 1;
        let row_len = 1_usize << row;
        let row = row as usize;

        if len <= 2 * row_len && row + 1 < self.buf.len() {
            if start.is_power_of_two() && end >> (row + 1) == 1 {
                return self.buf[row + 1][end].clone();
            }
            if (end + 1).is_power_of_two() && start >> (row + 1) == 0 {
                return self.buf[row + 1][start].clone();
            }
        }

        M::op(self.buf[row][start].clone(), self.buf[row][end].clone())
    }
}

impl<M> From<Vec<M::Set>> for DisjointSparseTable<M>
where
    M: Monoid,
    M::Set: Clone,
{
    fn from(base: Vec<M::Set>) -> Self {
        let len = base.len();

        let height = len.next_power_of_two().trailing_zeros().max(1) as usize;
        let mut buf = vec![base; height];

        for i in 1..height {
            let w = 1 << i;
            for j in (1..).step_by(2).take_while(|&j| j * w <= len) {
                let mid = j * w;
                for r in (1..w).take_while(|r| mid + r < len) {
                    buf[i][mid + r] = M::op(
                        buf[i][mid + r - 1].clone(),
                        buf[0][mid + r].clone(),
                    );
                }
            }
        }

        for i in 1..height {
            let w = 1 << i;
            for j in (1..).step_by(2).take_while(|&j| j * w <= len) {
                let mid = j * w - 1;
                for l in 1..w {
                    buf[i][mid - l] = if mid > l && (l + 1).is_power_of_two() {
                        let ei = (mid - l).trailing_zeros() as usize;
                        let ej = mid;
                        buf[ei][ej].clone()
                    } else {
                        M::op(
                            buf[0][mid - l].clone(),
                            buf[i][mid - l + 1].clone(),
                        )
                    };
                }
            }
        }
        Self { buf }
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
