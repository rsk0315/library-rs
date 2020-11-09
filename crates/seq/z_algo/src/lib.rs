use std::fmt::Debug;
use std::ops::Range;

use push_pop::{PopBack, PushBack};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZSearcher<T: Eq> {
    pat: Vec<T>,
    z: Vec<usize>,
}

impl<T: Clone + Eq> From<Vec<T>> for ZSearcher<T> {
    fn from(pat: Vec<T>) -> Self {
        todo!()
    }
}

impl<T: Eq> ZSearcher<T> {
    fn calc_z(&self, i: usize) -> usize {
        todo!()
    }

    pub fn occurrences<'a, S: 'a + AsRef<[T]>>(
        &'a self,
        s: S,
    ) -> Occurrences<T, S> {
        Occurrences {
            text_index: 0,
            pat_index: 0,
            z: &self,
            text: s,
        }
    }
}

pub struct Occurrences<'a, T: Eq, S: 'a + AsRef<[T]>> {
    text_index: usize,
    pat_index: usize,
    z: &'a ZSearcher<T>,
    text: S,
}

impl<T: Eq, S: AsRef<[T]>> Iterator for Occurrences<'_, T, S> {
    type Item = Range<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T: Eq> PushBack for ZSearcher<T> {
    type Input = T;
    fn push_back(&mut self, x: T) {
        todo!();
    }
}

impl<T: Eq> PopBack for ZSearcher<T> {
    type Output = usize;
    fn pop_back(&mut self) -> Option<usize> {
        todo!()
    }
}
