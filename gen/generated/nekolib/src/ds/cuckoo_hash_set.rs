//! `CuckooHashMap<K, ()>` の wrapper。

use super::cuckoo_hash_map;

use std::hash::Hash;
use std::iter::FromIterator;

use cuckoo_hash_map::CuckooHashMap;

/// `CuckooHashMap<K, ()>` の wrapper。
#[derive(Clone, Debug)]
pub struct CuckooHashSet<K>(CuckooHashMap<K, ()>);

impl<K: Eq + Hash> CuckooHashSet<K> {
    pub fn new() -> Self { Self(CuckooHashMap::new()) }
    pub fn contains(&self, key: &K) -> bool { self.0.contains_key(key) }
    pub fn insert(&mut self, key: K) -> bool {
        self.0.insert(key, ()).is_none()
    }
    pub fn remove(&mut self, key: &K) -> bool { self.0.remove(key).is_some() }
    pub fn len(&self) -> usize { self.0.len() }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

impl<K: Eq + Hash> FromIterator<K> for CuckooHashSet<K> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = K>,
    {
        let mut res = CuckooHashSet::new();
        for k in iter {
            res.insert(k);
        }
        res
    }
}

impl<K: Eq + Hash> Extend<K> for CuckooHashSet<K> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = K>,
    {
        for k in iter {
            self.insert(k);
        }
    }
}
