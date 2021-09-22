//! Cuckoo hashing による連想配列。

use std::collections::hash_map::RandomState;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash, Hasher};
use std::iter::FromIterator;

/// Cuckoo hashing による連想配列。
///
/// # Idea
/// `todo!()`
///
/// # References
/// - <http://web.stanford.edu/class/cs166/lectures/07/Slides07.pdf>
#[derive(Clone, Debug)]
pub struct CuckooHashMap<K, V> {
    rs: [RandomState; 2],
    buf: [Vec<Vec<(K, V)>>; 2],
    len: usize,
}

const LOOP_THRESHOLD: usize = 8;
const LEN_THRESHOLD: usize = 32;

impl<K: Clone + Eq + Hash, V: Clone> CuckooHashMap<K, V> {
    pub fn new() -> Self {
        let r0 = RandomState::new();
        let r1 = RandomState::new();
        Self {
            rs: [r0, r1],
            buf: [vec![vec![]], vec![vec![]]],
            len: 0,
        }
    }

    pub fn contains_key(&self, key: &K) -> bool { self.find(key).is_some() }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if let Some((i, j, k)) = self.find(&key) {
            let old = std::mem::replace(&mut self.buf[i][j][k].1, val);
            return Some(old);
        }

        if let Some(elts) = self.try_insert((key, val), LOOP_THRESHOLD) {
            self.rehash(elts);
        }
        self.len += 1;
        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.find(key).map(|(i, j, k)| {
            self.len -= 1;
            self.buf[i][j].swap_remove(k).1
        })
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

    fn try_insert(
        &mut self,
        mut keyval: (K, V),
        thresh: usize,
    ) -> Option<Vec<(K, V)>> {
        for _ in 0..thresh {
            for i in 0..=1 {
                let j = self.get_hash(&keyval.0, i);
                if self.buf[i][j].len() < LEN_THRESHOLD {
                    self.buf[i][j].push(keyval);
                    return None;
                }
                std::mem::swap(&mut keyval, self.buf[i][j].last_mut().unwrap());
            }
        }

        let mut elts = vec![keyval];
        elts.extend(self.buf[0].drain(..).flatten());
        elts.extend(self.buf[1].drain(..).flatten());
        Some(elts)
    }

    fn rehash(&mut self, mut elts: Vec<(K, V)>) {
        let len = Self::calc_len(elts.len());

        'outer: loop {
            for i in 0..=1 {
                self.buf[i] = vec![vec![]; len];
                self.rs[i] = RandomState::new();
            }
            while let Some((k, v)) = elts.pop() {
                if let Some(tmp) = self.try_insert((k, v), LOOP_THRESHOLD) {
                    elts.extend(tmp);
                    continue 'outer;
                }
            }
            return;
        }
    }

    fn calc_len(n: usize) -> usize { n }

    fn find(&self, key: &K) -> Option<(usize, usize, usize)> {
        (0..=1).find_map(|i| {
            let j = self.get_hash(key, i);
            self.buf[i][j]
                .iter()
                .enumerate()
                .find(|(_, (key0, _))| key0 == key)
                .map(|(k, _)| (i, j, k))
        })
    }

    fn get_hash(&self, key: &K, i: usize) -> usize {
        let mut h = self.rs[i].build_hasher();
        key.hash(&mut h);
        h.finish() as usize % self.buf[i].len()
    }
}

impl<K: Clone + Eq + Hash, V: Clone> FromIterator<(K, V)>
    for CuckooHashMap<K, V>
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let mut res = CuckooHashMap::new();
        for (k, v) in iter {
            res.insert(k, v);
        }
        res
    }
}

impl<K: Clone + Eq + Hash, V: Clone> Extend<(K, V)> for CuckooHashMap<K, V> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}
