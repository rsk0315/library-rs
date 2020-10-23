//! 尺取り法。

use super::super::traits::elastic_slice;

use elastic_slice::{ElasticSlice, ExpandBack, ShrinkFront, SliceHash};

/// 尺取り法で、各始端に対して境界を探す。
///
/// スライスと述語を引数に取り、スライスの各始端 $l$ に対して $r\_l$ を求める。
/// $r\_l$ は、以下の両方を満たす $r < n$ が存在すればその $r$、存在しなければ $n$ である。
/// $$ P(h([l, r))) \\text{ and } \\lnot P(h([l, r+1))). $$
///
/// ここで、$P$ は `pred`、$h([l, r))$ は `slice.start() == l`, `slice.end() == r`
/// における `slice.hash(())` によって計算される。
///
/// # Requirements
/// 各始端 $l$ に対して、$\\lnot P(h([l, m\_l)))$ なる $m\_l$
/// が存在するとき、次の二つが成り立つ。
/// - ${}^\\forall i\\in [l, m\_l)$ について $P(h([l, i)))$
/// - ${}^\\forall i\\in (m\_l, n)$ について $\\lnot P(h([l, i)))$
///
/// また、空区間に対しては $P$ は真を返す必要がある[^1]。
///
/// [^1]: そうでないと、返り値が定義しにくいためである。
/// $l-1$ や $-1$ が返り値として挙げられるが、後者では
/// `1_usize.wrapping_neg()` を使う必要がある上、大小関係がややこしくなり厄介。
///
/// # Complexity
/// `expand_back` および `shrink_front` の呼び出しを高々 $n$ 回、
/// `pred` の呼び出しを高々 $2n$ 回行う。
///
/// # Suggestions
/// [Examples] を見ての通り、構造体の宣言が大袈裟に感じられる。
/// 一方で、クロージャを渡すような設計でも綺麗にはならないと思われる。
///
/// 構造体を作るのが冗長に感じる程度の場合には、
/// これを使わずにインラインで書いてしまう方が楽そうに見えてしまう。
///
/// [Examples]: #examples
///
/// # Examples
/// ```
/// use nekolib::traits::{ElasticSlice, ExpandBack, ShrinkFront, SliceHash};
/// use nekolib::algo::window_bisect;
///
/// struct RangeSum {
///     buf: Vec<i32>,
///     start: usize,
///     end: usize,
///     sum: i32,
/// }
///
/// impl From<Vec<i32>> for RangeSum {
///     fn from(buf: Vec<i32>) -> Self {
///         Self { buf, start: 0, end: 0, sum: 0 }
///     }
/// }
///
/// impl ElasticSlice for RangeSum {
///     fn reset(&mut self) {
///         self.start = 0;
///         self.end = 0;
///         self.sum = 0;
///     }
///     fn full_len(&self) -> usize { self.buf.len() }
///     fn start(&self) -> usize { self.start }
///     fn end(&self) -> usize { self.end }
/// }
///
/// impl SliceHash for RangeSum {
///     type Salt = ();
///     type Hashed = i32;
///     fn hash(&self, _: ()) -> i32 { self.sum }
/// }
///
/// impl ExpandBack for RangeSum {
///     fn expand_back(&mut self) {
///         self.sum += self.buf[self.end];
///         self.end += 1;
///     }
/// }
///
/// impl ShrinkFront for RangeSum {
///     fn shrink_front(&mut self) {
///         self.sum -= self.buf[self.start];
///         self.start += 1;
///     }
/// }
///
/// let rs: RangeSum = vec![1, 4, 1, 4, 2, 1, 3, 5, 6].into();
/// assert_eq!(
///     window_bisect(rs, |x| x <= 5),
///     vec![2, 3, 4, 4, 6, 7, 7, 8, 8]
/// );
///
/// let rs: RangeSum = vec![6, 2, 5, 2, 3, 2, 1, 1, 1].into();
/// assert_eq!(
///     window_bisect(rs, |x| x <= 4),
///     vec![0, 2, 2, 4, 5, 8, 9, 9, 9]
/// );
/// ```
pub fn window_bisect<S, P>(mut slice: S, pred: P) -> Vec<usize>
where
    S: ElasticSlice + ExpandBack + ShrinkFront + SliceHash<Salt = ()>,
    S::Hashed: Clone,
    P: Fn(S::Hashed) -> bool,
{
    slice.reset();
    let n = slice.full_len();
    let mut res = vec![n; n];
    for l in 0..n {
        loop {
            let o = slice.hash(());
            if !pred(o) {
                res[l] = slice.end() - 1;
                break;
            }
            if slice.end() == n {
                return res;
            }
            slice.expand_back();
        }
        if slice.is_empty() {
            slice.expand_back();
        }
        slice.shrink_front();
    }
    res
}
