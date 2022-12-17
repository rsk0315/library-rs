pub trait Sssp<V> {
    type Cost;
    type Path: Iterator<Item = V>;
    fn cost(&self, dst: &V) -> Option<Self::Cost>;
    fn path(&self, dst: &V) -> Self::Path;
}
