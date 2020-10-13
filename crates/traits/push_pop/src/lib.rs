pub trait Push {
    type Input;
    fn push(&mut self, x: Self::Input);
}

pub trait PushFront {
    type Input;
    fn push_front(&mut self, x: Self::Input);
}

pub trait PushBack {
    type Input;
    fn push_back(&mut self, x: Self::Input);
}

pub trait Pop {
    type Output;
    fn pop(&mut self) -> Option<Self::Output>;
}

pub trait PopFront {
    type Output;
    fn pop_front(&mut self) -> Option<Self::Output>;
}

pub trait PopBack {
    type Output;
    fn pop_back(&mut self) -> Option<Self::Output>;
}
