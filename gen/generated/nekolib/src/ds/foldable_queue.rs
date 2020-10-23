//! fold 可能キュー。

use super::super::traits::binop;
use super::super::traits::fold;
use super::super::traits::push_pop;

use std::ops::RangeFull;

use binop::Monoid;
use fold::Fold;
use push_pop::{Pop, PopFront, Push, PushBack};

/// fold 可能キュー。
///
/// いわゆる SWAG (*S*liding *W*indow *Ag*gregation)。
/// モノイドを要素とするキューであって、全体のモノイド積を計算できる。
/// 逆元がある演算であれば、単に要素を一つ持って計算すればよい。
///
/// # Idea
/// スタックを二つ使ってキューを実現できることを応用する。
/// モノイド積を管理するスタックも二つ用意する。
/// 後者のスタックそれぞれの top が、対応する前者のスタック中の要素の総積となるように管理する。
/// これにより、キュー全体としての積は、二つの要素の積として計算できる。
///
/// # Implementation notes
/// `fold()` だけを考えると front stack には元の値を入れる必要はないが、
/// `pop()` の際の挙動を標準の [`VecDeque`] などと合わせたかったので、含めることにした。
/// fold した値を返すということにすれば含めなくて済むが、あまりそうしたくなかったので。
///
/// [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(1)$|
/// |`push` (`push_back`)|$\\Theta(1)$|
/// |`pop` (`pop_front`)|amortized $\\Theta(1)$|
/// |`fold`|$\\Theta(1)$|
///
/// `pop` の計算量は、two-stack queue のならし解析から従う。
/// わかりやすい資料として
/// [CS166](http://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/07/Small07.pdf)
/// を挙げておく。
///
/// # Examples
/// ```
/// use nekolib::ds::FoldableQueue;
/// use nekolib::traits::{Fold, Pop, Push};
/// use nekolib::utils::OpMin;
///
/// let mut fq = FoldableQueue::<OpMin<i32>>::new();
/// assert_eq!(fq.fold(..), std::i32::MAX);
/// fq.push(6);
/// assert_eq!(fq.fold(..), 6);
/// fq.push(3);
/// assert_eq!(fq.fold(..), 3);
/// fq.push(4);
/// assert_eq!(fq.fold(..), 3);
/// fq.pop();
/// assert_eq!(fq.fold(..), 3);
/// fq.pop();
/// assert_eq!(fq.fold(..), 4);
/// ```
#[derive(Debug)]
pub struct FoldableQueue<M: Monoid> {
    buf_front: Vec<M::Set>,
    buf_folded_front: Vec<M::Set>,
    buf_back: Vec<M::Set>,
    folded_back: M::Set,
}

impl<M: Monoid> FoldableQueue<M>
where
    M::Set: Clone,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            buf_front: vec![],
            buf_folded_front: vec![M::id()],
            buf_back: vec![],
            folded_back: M::id(),
        }
    }
    fn rotate(&mut self) {
        if self.buf_front.is_empty() {
            std::mem::swap(&mut self.buf_back, &mut self.buf_front);
            self.buf_front.reverse();
            self.build_folded();
        }
    }
    fn build_folded(&mut self) {
        self.folded_back = self
            .buf_back
            .iter()
            .fold(M::id(), |acc, x| M::op(acc, x.clone()));
        let n = self.buf_front.len();
        self.buf_folded_front = vec![M::id(); n + 1];
        for i in 0..n {
            self.buf_folded_front[i + 1] = M::op(
                self.buf_folded_front[i].clone(),
                self.buf_front[i].clone(),
            );
        }
    }
}

impl<M: Monoid> PushBack for FoldableQueue<M>
where
    M::Set: Clone,
{
    type Input = M::Set;
    fn push_back(&mut self, x: Self::Input) {
        self.folded_back =
            M::op(std::mem::replace(&mut self.folded_back, M::id()), x.clone());
        self.buf_back.push(x);
    }
}

impl<M: Monoid> Push for FoldableQueue<M>
where
    M::Set: Clone,
{
    type Input = M::Set;
    fn push(&mut self, x: Self::Input) {
        self.push_back(x);
    }
}

impl<M: Monoid> PopFront for FoldableQueue<M>
where
    M::Set: Clone,
{
    type Output = M::Set;
    fn pop_front(&mut self) -> Option<Self::Output> {
        self.rotate();
        if self.buf_folded_front.len() > 1 {
            self.buf_folded_front.pop();
        }
        self.buf_front.pop()
    }
}

impl<M: Monoid> Pop for FoldableQueue<M>
where
    M::Set: Clone,
{
    type Output = M::Set;
    fn pop(&mut self) -> Option<Self::Output> {
        self.pop_front()
    }
}

impl<M: Monoid> Fold<RangeFull> for FoldableQueue<M>
where
    M::Set: Clone,
{
    type Output = M;
    fn fold(&self, _: RangeFull) -> M::Set {
        let front = self.buf_folded_front.last().unwrap().clone();
        M::op(front, self.folded_back.clone())
    }
}

impl<M: Monoid> Default for FoldableQueue<M>
where
    M::Set: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
