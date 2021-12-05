//! 区間から値への対応づけ。

use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::ops::{
    Bound::{self, Excluded, Included, Unbounded},
    RangeBounds,
};

#[derive(Clone, Eq, PartialEq)]
pub struct Interval<T: Ord>(Bound<T>, Bound<T>);

impl<T: Ord + Debug> Debug for Interval<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Included(x) => write!(fmt, "[{:?}, ", x),
            Excluded(x) => write!(fmt, "({:?}, ", x),
            Unbounded => write!(fmt, "(-oo, "),
        }?;
        match &self.1 {
            Included(x) => write!(fmt, "{:?}]", x),
            Excluded(x) => write!(fmt, "{:?})", x),
            Unbounded => write!(fmt, "oo)"),
        }
    }
}

impl<T: Ord + Clone> Interval<T> {
    fn from_bounds<B: RangeBounds<T>>(bounds: B) -> Self {
        let start = match bounds.start_bound() {
            Included(x) => Included(x.clone()),
            Excluded(x) => Excluded(x.clone()),
            Unbounded => Unbounded,
        };
        let end = match bounds.end_bound() {
            Included(x) => Included(x.clone()),
            Excluded(x) => Excluded(x.clone()),
            Unbounded => Unbounded,
        };
        Self(start, end)
    }
}

impl<T: Ord> Interval<T> {
    pub fn inf(&self) -> Option<&T> {
        match &self.0 {
            Included(x) | Excluded(x) => Some(x),
            Unbounded => None,
        }
    }
    pub fn sup(&self) -> Option<&T> {
        match &self.1 {
            Included(x) | Excluded(x) => Some(x),
            Unbounded => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match (&self.0, &self.1) {
            (Unbounded, _) | (_, Unbounded) => false,
            (Included(lo), Included(hi)) => lo > hi,
            (Included(lo), Excluded(hi)) => lo >= hi,
            (Excluded(lo), Included(hi)) => lo >= hi,
            (Excluded(lo), Excluded(hi)) => lo >= hi,
        }
    }

    pub fn is_superset_of(&self, other: &Self) -> bool {
        if other.is_empty() {
            return true;
        }
        if self.is_empty() {
            return false;
        }

        // self.0 <= other.0
        match (self.inf(), other.inf()) {
            (Some(lhs), Some(rhs)) if lhs > rhs => return false,
            (Some(lhs), Some(rhs)) if lhs == rhs => {
                if let (Excluded(_), Included(_)) = (&self.0, &other.0) {
                    return false;
                }
            }
            (Some(_), None) => return false,
            _ => {}
        }

        // self.1 >= other.1
        match (self.sup(), other.sup()) {
            (Some(lhs), Some(rhs)) if lhs < rhs => return false,
            (Some(lhs), Some(rhs)) if lhs == rhs => {
                if let (Excluded(_), Included(_)) = (&self.1, &other.1) {
                    return false;
                }
            }
            (Some(_), None) => return false,
            _ => {}
        }

        true
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let (left, right) =
            if self < other { (self, other) } else { (other, self) };
        if let (Some(left_1), Some(right_0)) = (left.sup(), right.inf()) {
            if right_0 < left_1 {
                true
            } else if right_0 == left_1 {
                matches!((&right.0, &left.1), (Included(_), Included(_)))
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn has_no_gap_with(&self, other: &Self) -> bool {
        let (left, right) =
            if self < other { (self, other) } else { (other, self) };
        if let (Some(left_1), Some(right_0)) = (left.sup(), right.inf()) {
            if right_0 < left_1 {
                true
            } else if right_0 == left_1 {
                !matches!((&right.0, &left.1), (Excluded(_), Excluded(_)))
            } else {
                false
            }
        } else {
            true
        }
    }

    fn unite(&mut self, mut other: Self) {
        if !self.has_no_gap_with(&other) || self.is_superset_of(&other) {
            return;
        }
        if let (Some(lhs), Some(rhs)) = (self.inf(), other.inf()) {
            if (lhs == rhs
                && matches!((&self.0, &other.0), (Excluded(_), Included(_))))
                || lhs > rhs
            {
                self.0 = std::mem::replace(&mut other.0, Unbounded);
            }
        } else if other.0 == Unbounded {
            self.0 = Unbounded;
        }
        if let (Some(lhs), Some(rhs)) = (self.sup(), other.sup()) {
            if (lhs == rhs
                && matches!((&self.1, &other.1), (Excluded(_), Included(_))))
                || lhs < rhs
            {
                self.1 = std::mem::replace(&mut other.1, Unbounded);
            }
        } else if other.1 == Unbounded {
            self.1 = Unbounded;
        }
    }
}

impl<T: Ord> Ord for Interval<T> {
    /// `(self.lhs.0 <=> self.rhs.0).then_with(|| self.lhs.1 <=> self.rhs.1)`
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.inf(), other.inf()) {
            (Some(lhs), Some(rhs)) => {
                lhs.cmp(rhs).then_with(|| match (&self.0, &other.0) {
                    (Included(_), Excluded(_)) => Less,
                    (Excluded(_), Included(_)) => Greater,
                    _ => Equal,
                })
            }
            (Some(_), None) => Greater,
            (None, Some(_)) => Less,
            (None, None) => Equal,
        }
        .then_with(|| match (self.sup(), other.sup()) {
            (Some(lhs), Some(rhs)) => {
                lhs.cmp(rhs).then_with(|| match (&self.1, &other.1) {
                    (Included(_), Excluded(_)) => Greater,
                    (Excluded(_), Included(_)) => Less,
                    _ => Equal,
                })
            }
            (Some(_), None) => Less,
            (None, Some(_)) => Greater,
            (None, None) => Equal,
        })
    }
}

impl<T: Ord> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn toggle_bound<T: Ord>(b: Bound<T>) -> Bound<T> {
    match b {
        Included(x) => Excluded(x),
        Excluded(x) => Included(x),
        Unbounded => Unbounded,
    }
}

/// 区間から値への対応づけ。
#[derive(Clone, Eq, PartialEq)]
pub struct IntervalMap<K: Ord, V> {
    buf: BTreeMap<Interval<K>, V>,
}

impl<K: Ord + Debug, V: Debug> Debug for IntervalMap<K, V> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_map().entries(self.buf.iter()).finish()
    }
}

impl<K: Ord + Clone, V: Eq + Clone> IntervalMap<K, V> {
    pub fn new() -> Self { Self { buf: BTreeMap::new() } }

    pub fn is_empty(&self) -> bool { self.buf.is_empty() }

    pub fn insert<B: RangeBounds<K>>(&mut self, bounds: B, value: V) {
        let mut interval = Interval::from_bounds(bounds);
        if interval.is_empty() {
            return;
        }
        self.remove_subset(&interval);

        if let Some(s) = self.buf.range(..&interval).next_back() {
            let lk = s.0.clone();
            let lv = s.1.clone();
            if lk.is_superset_of(&interval) {
                if lv != value {
                    self.remove_one(interval.clone(), lk);
                    self.buf.insert(interval, value);
                }
                return;
            }
            if lv != value {
                self.remove_one(interval.clone(), lk);
            } else if interval.has_no_gap_with(&lk) {
                self.buf.remove(&lk);
                interval.unite(lk);
            }
        }
        if let Some(s) = self.buf.range(&interval..).next() {
            let rk = s.0.clone();
            let rv = s.1.clone();
            if rv != value {
                self.remove_one(interval.clone(), rk);
            } else if interval.has_no_gap_with(&rk) {
                self.buf.remove(&rk);
                interval.unite(rk);
            }
        }
        self.buf.insert(interval, value);
    }

    fn remove_subset(
        &mut self,
        interval: &Interval<K>,
    ) -> Vec<(Interval<K>, V)> {
        let rm: Vec<_> = self
            .buf
            .range(interval..)
            .take_while(|(k, _)| interval.is_superset_of(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (k, _) in &rm {
            self.buf.remove(k);
        }
        rm
    }

    fn remove_one(
        &mut self,
        new: Interval<K>,
        old: Interval<K>,
    ) -> Option<(Interval<K>, V)> {
        if !old.intersects(&new) {
            return None;
        }
        let v = self.buf.remove(&old).unwrap();
        if new.is_superset_of(&old) {
            //    [--- old ---]
            // [---    new    ---]
            return Some((old, v));
        }
        if new < old {
            //    [--- old ---]
            // [--- new ---]
            let Interval(old_0, old_1) = old;
            if new.1 != Unbounded {
                let new_1_t = toggle_bound(new.1.clone());
                let v = v.clone();
                self.buf.insert(Interval(new_1_t, old_1), v);
            }
            Some((Interval(old_0, new.1), v))
        } else if old.is_superset_of(&new) {
            // [---    old    ---]
            //    [--- new ---]
            let Interval(old_0, old_1) = old;
            let Interval(new_0, new_1) = new;
            if new_0 != Unbounded {
                let new_0_t = toggle_bound(new_0.clone());
                let v = v.clone();
                self.buf.insert(Interval(old_0, new_0_t), v);
            }
            if new_1 != Unbounded {
                let new_1_t = toggle_bound(new_1.clone());
                let v = v.clone();
                self.buf.insert(Interval(new_1_t, old_1), v);
            }
            Some((Interval(new_0, new_1), v))
        } else {
            // [--- old ---]
            //    [--- new ---]
            let Interval(old_0, old_1) = old;
            if new.0 != Unbounded {
                let new_0_t = toggle_bound(new.0.clone());
                let v = v.clone();
                self.buf.insert(Interval(old_0, new_0_t), v);
            }
            Some((Interval(new.0, old_1), v))
        }
    }

    pub fn remove<B: RangeBounds<K>>(
        &mut self,
        bounds: B,
    ) -> Vec<(Interval<K>, V)> {
        let interval = Interval::from_bounds(bounds);
        if interval.is_empty() {
            return vec![];
        }

        let mut res = self.remove_subset(&interval);
        if let Some(s) = self.buf.range(..&interval).next_back() {
            let lk = s.0.clone();
            res.extend(self.remove_one(interval.clone(), lk));
        }
        if let Some(s) = self.buf.range(&interval..).next() {
            let rk = s.0.clone();
            res.extend(self.remove_one(interval, rk));
        }
        res.sort_unstable_by(|l, r| l.0.cmp(&r.0));
        res
    }

    pub fn clear(&mut self) { self.buf.clear(); }

    pub fn covering<B: RangeBounds<K>>(
        &self,
        bounds: B,
    ) -> Option<(&Interval<K>, &V)> {
        let interval = Interval::from_bounds(bounds);
        if self.buf.is_empty() || interval.is_empty() {
            None
        } else {
            self.buf
                .range(..=&Interval(interval.0.clone(), Unbounded))
                .next_back()
                .filter(|r| r.0.is_superset_of(&interval))
        }
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&Interval<K>, &V)> + DoubleEndedIterator + '_
    {
        self.buf.iter()
    }
}

impl<'a, K: Ord, V: Eq> IntoIterator for &'a IntervalMap<K, V> {
    type Item = (&'a Interval<K>, &'a V);
    type IntoIter = std::collections::btree_map::Iter<'a, Interval<K>, V>;
    fn into_iter(self) -> Self::IntoIter { self.buf.iter() }
}

impl<K: Ord, V: Eq> IntoIterator for IntervalMap<K, V> {
    type Item = (Interval<K>, V);
    type IntoIter = std::collections::btree_map::IntoIter<Interval<K>, V>;
    fn into_iter(self) -> Self::IntoIter { self.buf.into_iter() }
}
