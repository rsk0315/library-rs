//! fold 可能キュー。

use std::ops::RangeFull;

use binop::Monoid;
use fold::Fold;
use push_pop::{Pop, PopFront, Push, PushBack};

/// fold 可能キュー。
///
/// モノイドのキューであって、全体のモノイド積を計算できる。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(1)$|
/// |`push` (`push_back`)|$\\Theta(1)$|
/// |`pop` (`pop_front`)|amortized $\\Theta(1)$|
/// |`fold`|$\\Theta(1)$|
///
/// # Examples
/// 逆元がない演算について処理できるのが強みです。
///
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
///
/// もちろん非可換でも問題ありません。
///
/// ```
/// use nekolib::{impl_assoc_val, impl_mod_int};
/// use nekolib::ds::FoldableQueue;
/// use nekolib::traits::{AssocVal, Fold, Pop, Push};
/// use nekolib::utils::{ModInt, OpRollHash};
///
/// impl_mod_int! { Mod1e9p7 => 1_000_000_007_i64 }
/// type Mi = ModInt<Mod1e9p7>;
/// impl_assoc_val! { Base<Mi> => Mi::from(123) }
/// type OpRh = OpRollHash::<Mi, Base>;
///
/// let val_from = |s| OpRh::val_from(s);
///
/// let mut fq = FoldableQueue::<OpRh>::new();
/// assert_eq!(fq.fold(..), val_from(""));
/// fq.push(val_from("abraca"));
/// fq.push(val_from("dabra"));
/// assert_eq!(fq.fold(..), val_from("abracadabra"));
/// fq.pop();
/// assert_eq!(fq.fold(..), val_from("dabra"));
/// fq.pop();
/// assert_eq!(fq.fold(..), val_from(""));
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
