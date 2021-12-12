use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Left<T> {
    NegInfinity,
    Closed(T),
    Open(T),
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Right<T> {
    Open(T),
    Closed(T),
    PosInfinity,
}

impl<T: Debug> Debug for Left<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Left::NegInfinity => write!(f, "(-oo"),
            Left::Closed(x) => write!(f, "[{:?}", x),
            Left::Open(x) => write!(f, "({:?}", x),
        }
    }
}

impl<T: Debug> Debug for Right<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Right::Open(x) => write!(f, "{:?})", x),
            Right::Closed(x) => write!(f, "{:?}]", x),
            Right::PosInfinity => write!(f, "oo)"),
        }
    }
}

impl<T> Left<T> {
    pub fn inner(&self) -> Option<&T> {
        match self {
            Left::Open(x) | Left::Closed(x) => Some(x),
            Left::NegInfinity => None,
        }
    }
}

impl<T> Right<T> {
    pub fn inner(&self) -> Option<&T> {
        match self {
            Right::PosInfinity => None,
            Right::Closed(x) | Right::Open(x) => Some(x),
        }
    }
}

impl<T: Ord> Ord for Left<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Left::{Closed, NegInfinity, Open};
        match (self, other) {
            (NegInfinity, NegInfinity) => Equal,
            (NegInfinity, _) => Less,
            (_, NegInfinity) => Greater,
            (Closed(x), Open(y)) if x == y => Less,
            (Open(x), Closed(y)) if x == y => Greater,
            _ => self.inner().unwrap().cmp(other.inner().unwrap()),
        }
    }
}

impl<T: Ord> PartialOrd for Left<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Ord for Right<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Right::{Closed, Open, PosInfinity};
        match (self, other) {
            (PosInfinity, PosInfinity) => Equal,
            (_, PosInfinity) => Less,
            (PosInfinity, _) => Greater,
            (Open(x), Closed(y)) if x == y => Less,
            (Closed(x), Open(y)) if x == y => Greater,
            _ => self.inner().unwrap().cmp(other.inner().unwrap()),
        }
    }
}

impl<T: Ord> PartialOrd for Right<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Interval<T> {
    left: Left<T>,
    right: Right<T>,
}

impl<T: Debug> Debug for Interval<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.left, self.right)
    }
}

impl<T: Ord> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.left.cmp(&other.left).then_with(|| self.right.cmp(&other.right))
    }
}

impl<T: Ord> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone> Interval<T> {
    pub fn from_bounds(b: impl RangeBounds<T>) -> Self {
        let left = match b.start_bound() {
            Unbounded => Left::NegInfinity,
            Included(x) => Left::Closed(x.clone()),
            Excluded(x) => Left::Open(x.clone()),
        };
        let right = match b.end_bound() {
            Excluded(x) => Right::Open(x.clone()),
            Included(x) => Right::Closed(x.clone()),
            Unbounded => Right::PosInfinity,
        };
        Self { left, right }
    }
}

impl<T: Ord> Interval<T> {
    pub fn inf(&self) -> Option<&T> { self.left.inner() }
    pub fn sup(&self) -> Option<&T> { self.right.inner() }

    pub fn is_empty(&self) -> bool {
        match (&self.left, &self.right) {
            (Left::NegInfinity, _) => false,
            (_, Right::PosInfinity) => false,
            (Left::Closed(x), Right::Closed(y)) => x > y,
            _ => self.inf().unwrap() >= self.sup().unwrap(),
        }
    }
    pub fn intersects(&self, other: &Self) -> bool {
        let (left, right) =
            if self < other { (self, other) } else { (other, self) };
        match (&right.left, &left.right) {
            (Left::NegInfinity, _) => true,
            (_, Right::PosInfinity) => true,
            (Left::Closed(x), Right::Closed(y)) => x <= y,
            _ => right.inf().unwrap() < left.sup().unwrap(),
        }
    }
    pub fn is_connected_with(&self, other: &Self) -> bool {
        let (left, right) =
            if self < other { (self, other) } else { (other, self) };
        match (&right.left, &left.right) {
            (Left::NegInfinity, _) => true,
            (_, Right::PosInfinity) => true,
            (Left::Open(x), Right::Open(y)) => x < y,
            _ => right.inf().unwrap() <= left.sup().unwrap(),
        }
    }
    pub fn is_subset_of(&self, other: &Self) -> bool {
        //      [--- self ---]
        // [------- other -------]
        other.left <= self.left && self.right <= other.right
    }
    pub fn is_superset_of(&self, other: &Self) -> bool {
        // [------- self -------]
        //    [--- other ---]
        self.left <= other.left && other.right <= self.right
    }

    pub fn intersection(self, other: Self) -> Option<Interval<T>> {
        let (left, right) =
            if self < other { (self, other) } else { (other, self) };
        Some(Interval { left: right.left, right: left.right })
            .filter(|it| !it.is_empty())
    }
    pub fn connection(self, other: Self) -> Option<Interval<T>> {
        let (left, right) =
            if self < other { (self, other) } else { (other, self) };
        if !left.is_connected_with(&right) {
            return None;
        }
        Some(Interval { left: left.left, right: right.right })
    }
}

impl<T: Ord + Clone> Interval<T> {
    pub fn intersection_minus(
        self,
        other: Self,
    ) -> (Option<Interval<T>>, Vec<Interval<T>>) {
        if !self.intersects(&other) {
            return (None, vec![self]);
        }
        if self.is_subset_of(&other) {
            return (Some(self), vec![]);
        }
        if self.is_superset_of(&other) {
            // [-----   self   ------]
            //     [-- other ---]
            // ======================
            // [mi][intersection][nus]
            let isx = other.clone();
            let Interval { left: ll, right: lr } = self;
            let Interval { left: rl, right: rr } = other;
            let minus_left = {
                let tmp = match rl {
                    Left::NegInfinity => None,
                    Left::Closed(x) => Some(Right::Open(x)),
                    Left::Open(x) => Some(Right::Closed(x)),
                };
                tmp.map(|right| Interval { left: ll, right })
                    .filter(|it| !it.is_empty())
            };
            let minus_right = {
                let tmp = match rr {
                    Right::Open(x) => Some(Left::Closed(x)),
                    Right::Closed(x) => Some(Left::Open(x)),
                    Right::PosInfinity => None,
                };
                tmp.map(|left| Interval { left, right: lr })
                    .filter(|it| !it.is_empty())
            };

            let isx = if isx.is_empty() { None } else { Some(isx) };
            let minus: Vec<_> =
                minus_left.into_iter().chain(minus_right).collect();
            return (isx, minus);
        }
        let swap = self > other;
        let (left, right) = if swap { (other, self) } else { (self, other) };
        // [--- self ---]
        //          [--- other ---]
        // ========================
        // [ minus ][isx]
        let Interval { left: ll, right: lr } = left;
        let Interval { left: rl, right: rr } = right;
        let minus = if swap {
            let left = match lr.clone() {
                Right::Open(x) => Left::Closed(x),
                Right::Closed(x) => Left::Open(x),
                Right::PosInfinity => unreachable!(),
            };
            Interval { left, right: rr }
        } else {
            let right = match rl.clone() {
                Left::NegInfinity => unreachable!(),
                Left::Closed(x) => Right::Open(x),
                Left::Open(x) => Right::Closed(x),
            };
            Interval { left: ll, right }
        };
        let isx = Interval { left: rl, right: lr };
        (Some(isx), vec![minus])
    }
}

pub struct IntervalMap<K, V> {
    inner: BTreeMap<Interval<K>, V>,
}

impl<K: Ord + Clone, V: Eq + Clone> IntervalMap<K, V> {
    pub fn new() -> Self { Self { inner: BTreeMap::new() } }

    pub fn is_empty(&self) -> bool { self.inner.is_empty() }

    pub fn insert<B: RangeBounds<K>>(&mut self, b: B, v: V) {
        let mut it = Interval::from_bounds(b);
        if it.is_empty() || self.contains_internal(&it, &v) {
            return;
        }
        self.remove_internal(it.clone());
        self.connect(&mut it, &v);
        self.inner.insert(it, v);
    }

    pub fn remove<B: RangeBounds<K>>(&mut self, b: B) -> Vec<(Interval<K>, V)> {
        let it = Interval::from_bounds(b);
        if it.is_empty() {
            return vec![];
        }
        self.remove_internal(it)
    }

    fn contains_internal(&self, it: &Interval<K>, v: &V) -> bool {
        // it の superset である区間が含まれており、その値が v なら true
        (self.inner.range(it..).next())
            .into_iter()
            .chain(self.inner.range(..it).next_back())
            .any(|(ki, vi)| ki.is_superset_of(it) && vi == v)
    }

    fn remove_internal(&mut self, it: Interval<K>) -> Vec<(Interval<K>, V)> {
        let mut rm = self.remove_subset_of(&it);
        rm.extend(self.remove_intersection_of(it));
        rm.sort_unstable_by(|l, r| l.0.cmp(&r.0));
        rm
    }

    fn remove_subset_of(&mut self, it: &Interval<K>) -> Vec<(Interval<K>, V)> {
        // it の subset である区間を削除し、それを返す
        let rm: Vec<_> = (self.inner.range(it..))
            .take_while(|(ki, _)| ki.is_subset_of(it))
            .chain(
                (self.inner.range(..it).rev())
                    .take_while(|(ki, _)| ki.is_subset_of(it)),
            )
            .map(|(ki, vi)| (ki.clone(), vi.clone()))
            .collect();
        for (k, _) in &rm {
            self.inner.remove(k);
        }
        rm
    }

    fn remove_intersection_of(
        &mut self,
        it: Interval<K>,
    ) -> Vec<(Interval<K>, V)> {
        // it と intersection を持つ区間を削除し、それを返す。
        // ただし、it の subset はすでに削除済みとする
        let mut rm = vec![];
        if let Some((ki, _)) = self.inner.range(&it..).next() {
            if it.intersects(ki) {
                let ki = ki.clone();
                let vi = self.inner.remove(&ki).unwrap();
                let (isx, minus) = ki.intersection_minus(it.clone());
                for k in minus {
                    self.inner.insert(k, vi.clone());
                }
                rm.push((isx.unwrap(), vi));
            }
        }
        if let Some((ki, _)) = self.inner.range(..&it).next_back() {
            if it.intersects(ki) {
                let ki = ki.clone();
                let vi = self.inner.remove(&ki).unwrap();
                let (isx, minus) = ki.intersection_minus(it);
                for k in minus {
                    self.inner.insert(k, vi.clone());
                }
                rm.push((isx.unwrap(), vi));
            }
        }
        rm
    }

    fn connect(&mut self, it: &mut Interval<K>, v: &V) {
        // it の両隣にくる区間の値が v であればそれらを削除し、
        // it につなげる。
        if let Some((ki, vi)) = self.inner.range(&*it..).next() {
            if vi == v && it.is_connected_with(ki) {
                let ki = ki.clone();
                self.inner.remove(&ki);
                it.right = ki.right;
            }
        }
        if let Some((ki, vi)) = self.inner.range(..&*it).next_back() {
            if vi == v && it.is_connected_with(ki) {
                let ki = ki.clone();
                self.inner.remove(&ki);
                it.left = ki.left;
            }
        }
    }

    pub fn superset_of<B: RangeBounds<K>>(
        &self,
        b: B,
    ) -> Option<(&Interval<K>, &V)> {
        let it = Interval::from_bounds(b);
        if self.inner.is_empty() || it.is_empty() {
            return None;
        }
        (self.inner.range(..&it).next_back().into_iter())
            .chain(self.inner.range(&it..).next())
            .find(|(ki, _)| ki.is_superset_of(&it))
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&Interval<K>, &V)> + DoubleEndedIterator + '_
    {
        self.inner.iter()
    }
}

impl<'a, K: Ord, V: Eq> IntoIterator for &'a IntervalMap<K, V> {
    type Item = (&'a Interval<K>, &'a V);
    type IntoIter = std::collections::btree_map::Iter<'a, Interval<K>, V>;
    fn into_iter(self) -> Self::IntoIter { self.inner.iter() }
}

impl<K: Ord, V: Eq> IntoIterator for IntervalMap<K, V> {
    type Item = (Interval<K>, V);
    type IntoIter = std::collections::btree_map::IntoIter<Interval<K>, V>;
    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
}

impl<K: Ord + Debug, V: Debug> Debug for IntervalMap<K, V> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_map().entries(self.inner.iter()).finish()
    }
}
