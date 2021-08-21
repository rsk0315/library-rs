//! bit set。

use std::ops::RangeBounds;

/// bit set。
#[derive(Clone, Eq, PartialEq)]
pub struct BitSet(Vec<u64>);

const WORD_SIZE: usize = 64;

impl BitSet {
    /// 初期化。
    pub fn new() -> Self { Self(vec![]) }

    /// $i$ を含むとき真を返す。
    pub fn contains(&self, i: usize) -> bool {
        let (large, small) = (i / WORD_SIZE, i % WORD_SIZE);
        *self.0.get(large).unwrap_or(&0) >> small & 1 == 1
    }

    fn _count(&self, _i: impl RangeBounds<usize>) -> bool { todo!() }

    /// $i$ を追加する。
    pub fn insert(&mut self, i: usize) {
        let (large, small) = (i / WORD_SIZE, i % WORD_SIZE);
        if large >= self.0.len() {
            self.0.resize(large + 1, 0);
        }
        self.0[large] |= 1 << small;
    }

    /// $i$ を削除する。
    pub fn remove(&mut self, i: usize) {
        let (large, small) = (i / WORD_SIZE, i % WORD_SIZE);
        self.0.get_mut(large).map(|x| *x &= !(1 << small));
    }

    pub fn iter(&self) -> Iter<'_> { Iter::new(&self) }
}

impl Extend<usize> for BitSet {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}

pub struct Iter<'a> {
    bs: &'a BitSet,
    large: usize,
    small: usize,
    rem: u64,
}

impl<'a> Iter<'a> {
    pub fn new(bs: &'a BitSet) -> Self {
        let &rem = bs.0.get(0).unwrap_or(&0);
        Self { bs: &bs, large: 0, small: 0, rem }
    }
    fn advance(&mut self) {
        while self.rem == 0 && self.large + 1 < self.bs.0.len() {
            self.large += 1;
            self.rem = self.bs.0[self.large];
        }
        if self.rem == 0 && self.large + 1 == self.bs.0.len() {
            self.large += 1;
            return;
        }
        self.small = self.rem.trailing_zeros() as usize;
        self.rem &= !(1 << self.small);
    }
}

impl Iterator for Iter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.advance();
        (self.large < self.bs.0.len())
            .then(|| self.large * WORD_SIZE + self.small)
    }
}
