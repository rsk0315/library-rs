//! 座標圧縮。

use std::collections::{BTreeMap, BTreeSet};

/// 座標圧縮。
///
/// # Examples
/// ```
/// use nekolib::algo::ordered_hash;
///
/// let oh = ordered_hash(&[0, 1, 3, 1, 5]);
/// assert_eq!(oh.len(), 4);
/// assert_eq!(oh[&0], 0);
/// assert_eq!(oh[&1], 1);
/// assert_eq!(oh[&3], 2);
/// assert_eq!(oh[&5], 3);
/// ```
pub fn ordered_hash<'a, K: Ord>(buf: &'a [K]) -> BTreeMap<&'a K, usize> {
    let set: BTreeSet<_> = buf.iter().collect();
    set.into_iter().zip(0..).collect()
}
