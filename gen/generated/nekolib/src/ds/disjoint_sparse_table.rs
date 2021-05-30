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
/// # Idea
/// 各 $k$ ($1\\le k\< \\log\_2(n)$) について、区間
/// $[i\\cdot 2\^k-j, i\\cdot 2\^k)$ および $[i\\cdot 2\^k, i\\cdot 2\^k+j)$
/// ($2\\le j\\le 2\^k$、$i$ は区間の終端が $n$ 以下になる各奇数)
/// におけるモノイド積を予め計算しておく。
/// 任意の区間は、上記の区間を高々 $2$ つ合わせることで表現できる。
///
/// # Implementation notes
/// 前処理では、異なる段で同じ区間のモノイド積を複数回計算するのを避けるための工夫をしている。
/// その処理のオーバーヘッドにより、モノイド積のコストが高くない場合は、
/// 毎回計算する方が高速かもしれない。クエリ処理についても同様の工夫をしている。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`from`|$\\Theta(n\\log(n))$|
/// |`fold`|$\\Theta(1)$|
///
/// # Precise analysis
/// 前処理におけるモノイド積の計算回数は以下の値で上から抑えられる。
/// $$ n\\cdot\\lceil{\\log\_2(n)-3}\\rceil + 2\\cdot\\lceil{\\log\_2(n)}\\rceil + 2. $$
///
/// これは、$n = 1000$ で $7022$ であり、
/// [Secret](http://s3-ap-northeast-1.amazonaws.com/data.cms.ioi-jp.org/open-2014/2014-open-d2-secret.pdf)
/// の「$n = 1000$ でクエリ $8000$ 回以下」に余裕を持って間に合う。
///
/// クエリ処理の際には、
/// 与えられた区間が前処理で計算した区間であるか、長さが $1$ 以下の場合は、
/// 新たにモノイド積は計算せずに答えを返す。
/// そうでない場合はちょうど $1$ 回のモノイド積を計算する。
///
/// ## More precise analysis
///
/// 前処理の実際の計算回数は、以下のコードにより $O(\\log(n))$ 時間で計算できるはず。
/// コード長が長いので隔離したいかも。
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
///         .sum()
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
/// use nekolib::{impl_assoc_val, impl_mod_int};
/// use nekolib::ds::DisjointSparseTable;
/// use nekolib::math::ModInt;
/// use nekolib::traits::{AssocVal, Fold};
/// use nekolib::utils::OpRollHash;
///
/// impl_mod_int! { Mod1e9p7 => 1_000_000_007_i64 }
/// type Mi = ModInt<Mod1e9p7>;
/// impl_assoc_val! { Base<Mi> => Mi::from(123) }
/// type OpRh = OpRollHash::<Mi, Base>;
///
/// let val_from = |s| OpRh::val_from(s);
///
/// let dst: DisjointSparseTable<OpRh> = vec![
///     val_from("abra"), val_from("cad"), val_from("abra")
/// ].into();
/// assert_eq!(dst.fold(1..=2), val_from("cadabra"));
/// assert_eq!(dst.fold(..), val_from("abracadabra"));
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
    fn index(&self, i: usize) -> &Self::Output { &self.buf[0][i] }
}
