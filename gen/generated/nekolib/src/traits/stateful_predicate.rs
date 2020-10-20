pub trait StatefulPred {
    type Input;
    fn count(&self) -> usize;
    fn next(&mut self);
    fn pred(&self, x: &Self::Input) -> bool;
    fn reset(&mut self);
}
