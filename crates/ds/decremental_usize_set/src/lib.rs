/// `usize` の decremental set。
///
///
pub struct DecrementalSet {
    len: usize,
    small: Vec<usize>,
    large: UnionFind,
}

impl DecrementalSet {
    /// $\\{0, 1, \\dots, n-1\\}$ で初期化。
    pub fn new(n: usize) -> Self {
        todo!();
    }

    pub fn len(&self) -> usize {
        todo!();
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    pub fn less(&self, n: usize) -> Option<usize> {}
    pub fn less_equal(&self, n: usize) -> Option<usize> {}
    pub fn greater(&self, n: usize) -> Option<usize> {}
    pub fn greater_equal(&self, n: usize) -> Option<usize> {}

    pub fn neighbors(&self, n: usize) -> [Option<usize>; 3] {}

    pub fn remove(&self, n: usize) {}
}
