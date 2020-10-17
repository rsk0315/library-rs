pub trait ExpandFront {
    fn expand_front(&mut self);
}

pub trait ExpandBack {
    fn expand_back(&mut self);
}

pub trait ShrinkFront {
    fn shrink_front(&mut self);
}

pub trait ShrinkBack {
    fn shrink_back(&mut self);
}

pub trait ElasticSlice {
    fn reset(&mut self);
    fn full_len(&self) -> usize;
    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn len(&self) -> usize {
        self.end() - self.start()
    }
    fn is_empty(&self) -> bool {
        self.start() == self.end()
    }
}

pub trait SliceHash {
    type Salt;
    type Hashed;
    fn hash(&self, x: Self::Salt) -> Self::Hashed;
}
