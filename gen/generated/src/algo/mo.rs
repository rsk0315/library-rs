//! Mo's algorithm。

use super::super::traits::elastic_slice;

use std::cmp::Ordering::Equal;
use std::ops::Range;

use elastic_slice::{
    ElasticSlice, ExpandBack, ExpandFront, ShrinkBack, ShrinkFront, SliceHash,
};

/// Mo's algorithm。
///
/// オフラインのクエリを $q$ 個処理する。
/// $i$ 番目のクエリは「区間 $[l\_i, r\_i)$ と値 $x\_i$ によって計算される値を求めよ」
/// ということを意味する。
///
/// # Complexity
/// 区間の全長 $n$、クエリ数 $q$、パラメータ $b$ に対して、
/// `shrink_front` と `expand_front` を高々 $q\\cdot b$ 回、
/// `shrink_back` と `expand_back` を高々 $n\^2/b$ 回行う。
///
/// # Hints
/// 相加・相乗平均の等号成立条件から $b = n\\cdot q\^{-1/2}$
/// とするのがよさげに思えるが、実際には手でパラメータを調整したくなることも多い。
/// そのため、引数に `None` を渡した場合は上記の値を用い、`Some(b)`
/// を渡した場合はその `b` を用いる実装にしている。
///
/// 定数での除算は最適化により乗算などに置き換えられることが期待されるので、
/// 予め最大ケースにおける $b$ を計算するなどして、その値を渡す方がいいかも。
/// 個人的には $224$ から $384$ くらいの大きさの $32$ の倍数に祈ることが多い。
///
/// # Examples
/// ```
/// use std::collections::BTreeMap;
///
/// use nekolib::traits::{
///     ElasticSlice, ExpandBack, ExpandFront, ShrinkBack,
///     ShrinkFront, SliceHash,
/// };
/// use nekolib::algo::mo;
///
/// struct RangeDistinct {
///     buf: Vec<i32>,
///     start: usize,
///     end: usize,
///     count: BTreeMap<i32, usize>,
/// }
///
/// impl From<Vec<i32>> for RangeDistinct {
///     fn from(buf: Vec<i32>) -> Self {
///         Self { buf, start: 0, end: 0, count: BTreeMap::new() }
///     }
/// }
///
/// impl ElasticSlice for RangeDistinct {
///     fn reset(&mut self) {
///         self.start = 0;
///         self.end = 0;
///         self.count.clear();
///     }
///     fn full_len(&self) -> usize { self.buf.len() }
///     fn start(&self) -> usize { self.start }
///     fn end(&self) -> usize { self.end }
/// }
///
/// /// 区間 `start..end` に含まれる整数の集合と、`x` のみからなる
/// /// 集合との和集合の要素数を返す。
/// impl SliceHash for RangeDistinct {
///     type Salt = i32;
///     type Hashed = usize;
///     fn hash(&self, x: i32) -> usize {
///         self.count.len()
///             + if self.count.contains_key(&x) { 0 } else { 1 }
///     }
/// }
///
/// impl ExpandBack for RangeDistinct {
///     fn expand_back(&mut self) {
///         let k = self.buf[self.end];
///         *self.count.entry(k).or_insert(0) += 1;
///         self.end += 1;
///     }
/// }
///
/// impl ExpandFront for RangeDistinct {
///     fn expand_front(&mut self) {
///         self.start -= 1;
///         let k = self.buf[self.start];
///         *self.count.entry(k).or_insert(0) += 1;
///     }
/// }
///
/// impl ShrinkBack for RangeDistinct {
///     fn shrink_back(&mut self) {
///         self.end -= 1;
///         let k = self.buf[self.end];
///         match self.count.get_mut(&k) {
///             Some(x) if x == &1 => { self.count.remove(&k); }
///             Some(x) => *x -= 1,
///             None => unreachable!(),
///         }
///     }
/// }
///
/// impl ShrinkFront for RangeDistinct {
///     fn shrink_front(&mut self) {
///         let k = self.buf[self.start];
///         match self.count.get_mut(&k) {
///             Some(x) if x == &1 => { self.count.remove(&k); }
///             Some(x) => *x -= 1,
///             None => unreachable!(),
///         }
///         self.start += 1;
///     }
/// }
///
/// let rd: RangeDistinct = vec![1, 4, 1, 4, 2, 1, 3, 5, 6].into();
/// let qs = vec![(0..4, 1), (0..4, 2), (2..6, 1), (3..9, 2)];
/// assert_eq!(mo(rd, qs, Some(4)), vec![2, 3, 3, 6]);
/// ```
pub fn mo<S>(
    mut slice: S,
    q: Vec<(Range<usize>, S::Salt)>,
    b: Option<usize>,
) -> Vec<S::Hashed>
where
    S: ElasticSlice
        + ExpandFront
        + ExpandBack
        + ShrinkFront
        + ShrinkBack
        + SliceHash,
    S::Hashed: Clone,
{
    let b = match b {
        Some(b) => b,
        None => todo!("sqrt(usize) is to be implemented"),
    };
    let qn = q.len();
    let mut q: Vec<_> = q.into_iter().enumerate().collect();
    q.sort_unstable_by(|(_, (ir, _)), (_, (jr, _))| {
        let Range { start: is, end: ie } = ir;
        let Range { start: js, end: je } = jr;
        let ib = is / b;
        let jb = js / b;
        match ib.cmp(&jb) {
            Equal if ib % 2 == 0 => ie.cmp(&je),
            Equal => je.cmp(&ie),
            c => c,
        }
    });

    let mut res = vec![None; qn];
    slice.reset();
    for (i, (Range { start: ql, end: qr }, x)) in q {
        while slice.end() < qr {
            slice.expand_back();
        }
        while slice.start() > ql {
            slice.expand_front();
        }
        while slice.start() < ql {
            slice.shrink_front();
        }
        while slice.end() > qr {
            slice.shrink_back();
        }
        res[i] = Some(slice.hash(x));
    }
    res.into_iter().map(std::option::Option::unwrap).collect()
}
