use binop::Magma;

pub trait FoldBisect {
    type Input: Magma;
    fn fold_bisect<F>(&self, i: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<Self::Input as Magma>::Set) -> bool;
}

pub trait FoldBisectRev {
    type Input: Magma;
    fn fold_bisect_rev<F>(&self, i: usize, pred: F) -> Option<usize>
    where
        F: Fn(&<Self::Input as Magma>::Set) -> bool;
}
