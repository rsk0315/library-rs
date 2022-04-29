//! 双方向連想配列。

use std::borrow::Borrow;
use std::collections::{btree_map::Range, BTreeMap};
use std::fmt::Debug;
use std::ops::RangeBounds;

/// 双方向連想配列。
///
/// $k\\mapsto v$ ではなく、全単射となるように $k\_l\\mapsto k\_r$ と
/// $k\_r\\mapsto k\_l$ を管理する。
///
/// # Examples
/// ```
/// use nekolib::ds::BTreeBimap;
///
/// let mut bimap = BTreeBimap::new();
///
/// bimap.insert(1, 'a');
/// bimap.insert(2, 'b');
/// bimap.insert(3, 'c');
///
/// bimap.insert(1, 'b');
/// assert_eq!(bimap.len(), 2); // {1: 'b', 3, 'c'}
///
/// bimap.remove_left(&1);
/// bimap.remove_right(&'c');
/// assert!(bimap.is_empty());
/// ```
#[derive(Clone, Debug, Default)]
pub struct BTreeBimap<L: Ord, R: Ord> {
    left: BTreeMap<L, R>,
    right: BTreeMap<R, L>,
}

impl<L: Clone + Ord, R: Clone + Ord> BTreeBimap<L, R> {
    pub fn new() -> Self {
        Self { left: BTreeMap::new(), right: BTreeMap::new() }
    }
    pub fn is_empty(&self) -> bool { self.left.is_empty() }
    pub fn len(&self) -> usize { self.left.len() }
    pub fn insert(&mut self, l: L, r: R) {
        if let Some(old_r) = self.left.insert(l.clone(), r.clone()) {
            self.right.remove(&old_r);
        }
        if let Some(old_l) = self.right.insert(r, l) {
            self.left.remove(&old_l);
        }
    }
    pub fn remove_left(&mut self, l: &L) {
        if let Some(old_r) = self.left.remove(l) {
            self.right.remove(&old_r);
        }
    }
    pub fn remove_right(&mut self, r: &R) {
        if let Some(old_l) = self.right.remove(r) {
            self.left.remove(&old_l);
        }
    }

    // {iter|entry}_{left|right} とかも欲しいよね
    // .or_insert() は両方に入れたりする必要があるので注意。

    pub fn range_left<T, B>(&self, range: B) -> Range<'_, L, R>
    where
        T: Ord,
        L: Borrow<T>,
        B: RangeBounds<T>,
    {
        self.left.range(range)
    }
    pub fn range_right<T, B>(&self, range: B) -> Range<'_, R, L>
    where
        T: Ord,
        R: Borrow<T>,
        B: RangeBounds<T>,
    {
        self.right.range(range)
    }
}

#[test]
fn test_eq() {
    let mut b = BTreeBimap::new();
    b.insert(2, 20);
    b.insert(2, 20);
    assert_eq!(b.len(), 1);
}
