//! イテレータのグルーピング。

/// イテレータのグルーピング。
///
/// # See also
/// [`GroupBy`]
///
/// [`GroupBy`]: trait.GroupBy.html
pub trait UsizeGroupBy<V> {
    /// # Examples
    /// ```
    /// use nekolib::traits::UsizeGroupBy;
    ///
    /// let a = vec![1, 4, 3, -5, -6, 0, 2, -2, 3];
    /// let g = a.iter().copied().usize_group_by(|&ai: &i32| ai.rem_euclid(3) as usize);
    ///
    /// assert_eq!(g.len(), 3);
    /// assert_eq!(g[0], [3, -6, 0, 3]);
    /// assert_eq!(g[1], [1, 4, -5, -2]);
    /// assert_eq!(g[2], [2]);
    /// ```
    fn usize_group_by(self, index: impl FnMut(&V) -> usize) -> Vec<Vec<V>>;
}

impl<V, I: Iterator<Item = V>> UsizeGroupBy<V> for I {
    fn usize_group_by(self, mut index: impl FnMut(&V) -> usize) -> Vec<Vec<V>> {
        let mut res: Vec<Vec<_>> = vec![];
        for v in self {
            let i = index(&v);
            if i >= res.len() {
                res.resize_with(i + 1, Default::default);
            }
            res[i].push(v);
        }
        res
    }
}
