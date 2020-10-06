use binop::{Magma, Monoid};
use fold::Fold;

pub trait FoldBisect: Fold {
    fn fold_bisect(&self, i: usize, pred: F) -> usize
    where
        F: Fn(<Self as Fold>::Output) -> bool;
}

pub trait FoldBisectRev: Fold {
    fn fold_bisect_rev(&self, i: usize, pred: F) -> usize
    where
        F: Fn(<Self as Fold>::Output) -> bool;
}
