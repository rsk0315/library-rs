//! 区間に関する関数に関するトレイト。

use std::cmp::Ordering::Equal;
use std::ops::Range;

/// 区間に関する関数。
///
/// 典型的には [`Mo`] で用いる。
/// 同じ引数を与えた場合でも、区間の状態によって返り値が異なる。
/// 区間が空のときに縮める操作は呼び出されないことを仮定してよい（はず）。
pub trait RangeHash {
    /// 関数の引数の型。
    type Input;
    /// 関数の返り値の型。
    type Output: Clone;
    /// 先頭の添字を 0-indexed の閉区間として返す。
    fn start(&self) -> usize;
    /// 末尾の添字を 0-indexed の開区間として返す。
    fn end(&self) -> usize;
    /// 区間の全長を返す。
    fn full_len(&self) -> usize;
    /// 区間を $[0, 0)$ に戻す。
    fn reset(&mut self);
    /// 先頭を伸ばす。
    fn shrink_front(&mut self);
    /// 末尾を伸ばす。
    fn shrink_back(&mut self);
    /// 先頭を縮める。
    fn expand_front(&mut self);
    /// 末尾を縮める。
    fn expand_back(&mut self);
    /// 引数 `x` と現在の区間から定義されるハッシュ値を返す。
    fn hash(&self, x: Self::Input) -> Self::Output;

    fn batch_query(
        &mut self,
        qs: Vec<(Range<usize>, Self::Input)>,
        b: Option<usize>,
    ) -> Vec<Self::Output> {
        let b = match b {
            Some(b) => b,
            None => todo!(), // self.full_len().sqrt(),
        };
        let qn = qs.len();
        let mut qs: Vec<_> = qs.into_iter().enumerate().collect();
        qs.sort_unstable_by(|(_, (ir, _)), (_, (jr, _))| {
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
        self.reset();
        for (i, (Range { start: ql, end: qr }, x)) in qs {
            while self.end() < qr {
                self.expand_back();
            }
            while self.start() > ql {
                self.expand_front();
            }
            while self.start() < ql {
                self.shrink_front();
            }
            while self.end() > qr {
                self.shrink_back();
            }
            res[i] = Some(self.hash(x));
        }
        res.into_iter().map(std::option::Option::unwrap).collect()
    }
}
