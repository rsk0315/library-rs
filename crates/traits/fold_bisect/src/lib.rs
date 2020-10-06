use binop::Magma;

pub trait FoldBisect {
    type Folded: Magma;
    fn fold_bisect<F>(&self, i: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<Self::Folded as Magma>::Set) -> bool;
}

pub trait FoldBisectRev {
    type Folded: Magma;
    fn fold_bisect_rev<F>(&self, i: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<Self::Folded as Magma>::Set) -> bool;
}
