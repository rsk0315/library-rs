//! 多重集合。

use std::collections::{btree_map::Iter as BTreeMapIter, BTreeMap};
use std::fmt::{self, Debug};

/// 多重集合。
pub struct BTreeMultiset<K>(BTreeMap<K, usize>, usize);

impl<K: Ord> BTreeMultiset<K> {
    pub fn new() -> Self { Self(BTreeMap::new(), 0) }
    pub fn insert(&mut self, k: K) { self.insert_n(k, 1) }
    pub fn insert_n(&mut self, k: K, n: usize) {
        if n == 0 {
            return;
        }
        *self.0.entry(k).or_insert(0) += n;
        self.1 += n;
    }
    pub fn remove(&mut self, k: &K) { self.remove_n(k, 1) }
    pub fn remove_n(&mut self, k: &K, n: usize) {
        if n == 0 {
            return;
        }
        match self.0.get(k) {
            Some(&c) if c <= n => {
                self.0.remove(&k);
                self.1 -= c;
            }
            Some(_) => {
                *self.0.get_mut(k).unwrap() -= n;
                self.1 -= n;
            }
            None => {}
        }
    }
    pub fn min(&self) -> Option<&K> { self.0.keys().next() }
    pub fn max(&self) -> Option<&K> { self.0.keys().next_back() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn len(&self) -> usize { self.1 }

    pub fn iter(&self) -> Iter<'_, K> { Iter::new(self) }
}

pub struct Iter<'a, K: 'a> {
    iter: BTreeMapIter<'a, K, usize>,
    key: Option<&'a K>,
    count: usize,
}

impl<'a, K: 'a + Ord> Iter<'a, K> {
    pub fn new(ms: &'a BTreeMultiset<K>) -> Self {
        let mut iter = ms.0.iter();
        if let Some((key, &count)) = iter.next() {
            Self { iter, key: Some(key), count }
        } else {
            Self { iter, key: None, count: 0 }
        }
    }
}

impl<'a, K: 'a> Iterator for Iter<'a, K> {
    type Item = &'a K;
    fn next(&mut self) -> Option<&'a K> {
        if self.key.is_none() {
            return None;
        }

        if self.count > 0 {
            self.count -= 1;
        } else if let Some((k, &v)) = self.iter.next() {
            self.key = Some(k);
            self.count = v - 1;
        } else {
            self.key = None;
        }
        self.key
    }
}

impl<'a, K: Ord> IntoIterator for &'a BTreeMultiset<K> {
    type Item = &'a K;
    type IntoIter = Iter<'a, K>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<K: Debug> Debug for BTreeMultiset<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

#[test]
fn test_debug_fmt() {
    let mut ms = BTreeMultiset::new();
    ms.insert(5);
    ms.insert(5);
    ms.insert(7);
    ms.insert_n(4, 0);
    ms.remove_n(&3, 0);
    assert_eq!(format!("{:?}", ms), "{5: 2, 7: 1}");
}

#[test]
fn test_iter() {
    let mut ms = BTreeMultiset::new();
    ms.insert(5);
    ms.insert(5);
    ms.insert(6);
    ms.insert(6);
    ms.insert(6);
    ms.insert(7);
    ms.insert_n(4, 0);
    ms.remove_n(&3, 0);
    assert_eq!(ms.iter().copied().collect::<Vec<_>>(), [5, 5, 6, 6, 6, 7]);
}

#[test]
fn test_count() {
    let mut ms = BTreeMultiset::new();
    assert_eq!(ms.len(), 0);

    ms.insert_n(5, 2);
    assert_eq!(ms.len(), 2);

    ms.remove_n(&5, 3);
    assert_eq!(ms.len(), 0);
}
