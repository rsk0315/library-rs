use std::ops::{Deref, DerefMut};

pub trait GetMut<'a> {
    type Output: Deref + DerefMut;
    fn get_mut(&'a mut self, i: usize) -> Option<Self::Output>;
}
