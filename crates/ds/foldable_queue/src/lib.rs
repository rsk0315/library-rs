use std::ops::RangeFull;

use binop::*;
use fold::Fold;

pub struct FoldableQueue<M: Clone + Monoid> {
    buf_front: Vec<M>,
    buf_folded_front: Vec<M>,
    buf_back: Vec<M>,
    folded_back: M,
}

impl<M: Clone + Monoid> FoldableQueue<M> {
    fn new() -> Self {
        Self {
            buf_front: vec![],
            buf_folded_front: vec![],
            buf_back: vec![],
            folded_back: M::id(),
        }
    }
    fn rotate(&mut self) {}
}

impl<M: Clone + Monoid> PushBack for FoldableQueue<M> {
    fn push_back(&mut self, x: M) {
        todo!()
    }
}

impl<M: Clone + Monoid> PopFront for FoldableQueue<M> {
    fn pop_front(&mut self, x: M) -> M {
        todo!()
    }
}

impl<M: Clone + Monoid> Fold<RangeFull> for FoldableQueue<M> {
    fn fold(&self, _: RangeFull) -> M {
        todo!()
    }
}
