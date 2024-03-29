//! fold 可能両端キュー。

use std::ops::RangeFull;

use binop::Monoid;
use fold::Fold;
use push_pop::{PopBack, PopFront, PushBack, PushFront};

/// fold 可能両端キュー。
///
/// モノイドを要素とする両端キューであって、全体のモノイド積を計算できる。
/// 逆元がある演算であれば、単に要素を一つ持って計算すればよい。
///
/// # Idea
/// スタックを二つ使って両端キューを実現できることを応用する[^1]。
/// モノイド積に関する方針は [`FoldableQueue`] と同じ。
///
/// [^1]: 内部的には `std::VecDeque` を使ってスタックを実現しているため、
/// 両端キュー二つで両端キューを実現していることになっている。ギャグ？
///
/// [`FoldableQueue`]: struct.FoldableQueue.html
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(1)$|
/// |`push_back`, `push_front`|$\\Theta(1)$|
/// |`pop_back`, `pop_front`|amortized $\\Theta(1)$|
/// |`fold`|$\\Theta(1)$|
///
/// キューを実現する場合と異なり、pop したい側の stack が空の際は、
/// 他方の stack から半分（奇数なら切り上げ）だけ要素をもらってくるようにする。
/// 各スタックの要素数の差の絶対値をポテンシャルとすることで、ならし定数時間が示せる。
///
/// # Examples
/// ```
/// use nekolib::ds::FoldableDeque;
/// use nekolib::traits::{Fold, PopBack, PopFront, PushBack, PushFront};
/// use nekolib::utils::OpMin;
///
/// let mut fq = FoldableDeque::<OpMin<i32>>::new();
/// assert_eq!(fq.fold(..), std::i32::MAX);
/// fq.push_back(6);
/// assert_eq!(fq.fold(..), 6);
/// fq.push_back(3);
/// assert_eq!(fq.fold(..), 3);
/// fq.push_front(4);
/// assert_eq!(fq.fold(..), 3);
/// fq.pop_back();
/// assert_eq!(fq.fold(..), 4);
/// fq.pop_front();
/// assert_eq!(fq.fold(..), 6);
/// fq.pop_back();
/// assert_eq!(fq.fold(..), std::i32::MAX);
/// ```
#[derive(Clone, Debug)]
pub struct FoldableDeque<M: Monoid> {
    buf_front: Vec<M::Set>,
    buf_folded_front: Vec<M::Set>,
    buf_back: Vec<M::Set>,
    buf_folded_back: Vec<M::Set>,
    monoid: M,
}

impl<M: Monoid> FoldableDeque<M>
where
    M::Set: Clone,
{
    #[must_use]
    pub fn new() -> Self
    where
        M: Default,
    {
        Self::with(M::default())
    }
    #[must_use]
    pub fn with(monoid: M) -> Self {
        Self {
            buf_front: vec![],
            buf_folded_front: vec![monoid.id()],
            buf_back: vec![],
            buf_folded_back: vec![monoid.id()],
            monoid,
        }
    }
    fn rotate_to_front(&mut self) {
        if !self.buf_front.is_empty() {
            return;
        }
        let n = (self.buf_back.len() + 1) / 2;
        let tmp_back = self.buf_back.split_off(n);
        self.buf_front = self.buf_back.split_off(0);
        self.buf_front.reverse();
        self.buf_back = tmp_back;
        self.build_folded();
    }
    fn rotate_to_back(&mut self) {
        if !self.buf_back.is_empty() {
            return;
        }
        let n = (self.buf_front.len() + 1) / 2;
        let tmp_front = self.buf_front.split_off(n);
        self.buf_back = self.buf_front.split_off(0);
        self.buf_back.reverse();
        self.buf_front = tmp_front;
        self.build_folded();
    }
    fn build_folded(&mut self) {
        {
            // front
            let n = self.buf_front.len();
            self.buf_folded_front = vec![self.monoid.id(); n + 1];
            for i in 0..n {
                self.buf_folded_front[i + 1] = self.monoid.op(
                    self.buf_front[i].clone(),
                    self.buf_folded_front[i].clone(),
                );
            }
        }
        {
            // back
            let n = self.buf_back.len();
            self.buf_folded_back = vec![self.monoid.id(); n + 1];
            for i in 0..n {
                self.buf_folded_back[i + 1] = self.monoid.op(
                    self.buf_folded_back[i].clone(),
                    self.buf_back[i].clone(),
                );
            }
        }
    }
}

impl<M: Monoid> PushBack for FoldableDeque<M>
where
    M::Set: Clone,
{
    type Input = M::Set;
    fn push_back(&mut self, x: Self::Input) {
        self.buf_back.push(x.clone());
        self.buf_folded_back.push(
            self.monoid.op(self.buf_folded_back.last().unwrap().clone(), x),
        );
    }
}

impl<M: Monoid> PushFront for FoldableDeque<M>
where
    M::Set: Clone,
{
    type Input = M::Set;
    fn push_front(&mut self, x: Self::Input) {
        self.buf_front.push(x.clone());
        self.buf_folded_front.push(
            self.monoid.op(x, self.buf_folded_front.last().unwrap().clone()),
        );
    }
}

impl<M: Monoid> PopBack for FoldableDeque<M>
where
    M::Set: Clone,
{
    type Output = M::Set;
    fn pop_back(&mut self) -> Option<Self::Output> {
        self.rotate_to_back();
        if self.buf_folded_back.len() > 1 {
            self.buf_folded_back.pop();
        }
        self.buf_back.pop()
    }
}

impl<M: Monoid> PopFront for FoldableDeque<M>
where
    M::Set: Clone,
{
    type Output = M::Set;
    fn pop_front(&mut self) -> Option<Self::Output> {
        self.rotate_to_front();
        if self.buf_folded_front.len() > 1 {
            self.buf_folded_front.pop();
        }
        self.buf_front.pop()
    }
}

impl<M: Monoid> Fold<RangeFull> for FoldableDeque<M>
where
    M::Set: Clone,
{
    type Output = M;
    fn fold(&self, _: RangeFull) -> M::Set {
        let front = self.buf_folded_front.last().unwrap().clone();
        let back = self.buf_folded_back.last().unwrap().clone();
        self.monoid.op(front, back)
    }
}

impl<M: Monoid> Default for FoldableDeque<M>
where
    M: Default,
    M::Set: Clone,
{
    fn default() -> Self { Self::new() }
}
