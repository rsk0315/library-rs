//! イテレータのグルーピング。

use std::collections::BTreeMap;

/// イテレータのグルーピング。
///
/// # Suggestions
/// `BTreeMap` と `HashMap` 用で分けるべき？ メソッド名を冗長にしたくない。
pub trait GroupBy<V> {
    /// # Examples
    /// ```
    /// use nekolib::traits::GroupBy;
    ///
    /// let a = vec![1, 4, 3, -5, -6, 0, 2, -2, 3];
    /// let g1 = a.iter().copied().group_by(|&ai: &i32| ai.rem_euclid(3));
    /// let g2 = a.iter().copied().group_by(|&ai: &i32| ai % 3);
    ///
    /// assert_eq!(g1.len(), 3);
    /// assert_eq!(g1[&0], [3, -6, 0, 3]);
    /// assert_eq!(g1[&1], [1, 4, -5, -2]);
    /// assert_eq!(g1[&2], [2]);
    ///
    /// assert_eq!(g2.len(), 4);
    /// assert_eq!(g2[&-2], [-5, -2]);
    /// assert_eq!(g2[&0], [3, -6, 0, 3]);
    /// assert_eq!(g2[&1], [1, 4]);
    /// assert_eq!(g2[&2], [2]);
    /// ```
    fn group_by<K: Ord>(self, key: impl FnMut(&V) -> K) -> BTreeMap<K, Vec<V>>;
}

impl<V, I: Iterator<Item = V>> GroupBy<V> for I {
    fn group_by<K: Ord>(
        self,
        mut key: impl FnMut(&V) -> K,
    ) -> BTreeMap<K, Vec<V>> {
        let mut res = BTreeMap::new();
        for v in self {
            let k = key(&v);
            res.entry(k).or_insert(vec![]).push(v);
        }
        res
    }
}
