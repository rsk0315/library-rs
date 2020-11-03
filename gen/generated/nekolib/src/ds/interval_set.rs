//! 区間の集合。

use std::cmp::Ordering::{self, *};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::ops::{
    Bound::{self, *},
    RangeBounds,
};

#[derive(Clone, Debug, Eq)]
struct Interval<T: Ord>(Bound<T>, Bound<T>);

impl<T: Clone + Ord> From<(Bound<&T>, Bound<&T>)> for Interval<T> {
    fn from(r: (Bound<&T>, Bound<&T>)) -> Interval<T> {
        let s = match r.0 {
            Included(lo) => Included(lo.clone()),
            Excluded(lo) => Excluded(lo.clone()),
            Unbounded => Unbounded,
        };
        let e = match r.1 {
            Included(hi) => Included(hi.clone()),
            Excluded(hi) => Excluded(hi.clone()),
            Unbounded => Unbounded,
        };
        Interval(s, e)
    }
}

impl<T: Ord> Interval<T> {
    fn is_empty(&self) -> bool {
        match (&self.0, &self.1) {
            (Unbounded, _) | (_, Unbounded) => false,
            (Included(lo), Included(hi)) => !(lo <= hi),
            (Included(lo), Excluded(hi))
            | (Excluded(lo), Included(hi))
            | (Excluded(lo), Excluded(hi)) => !(lo < hi),
        }
    }
    fn is_superset(&self, other: &Self) -> bool {
        match self.cmp(&other) {
            Less if self.1 == other.1 => true,
            Less => match (&self.1, &other.1) {
                (_, Unbounded) => false,
                (Unbounded, _) => true,
                (Excluded(lhs), Included(rhs)) if lhs == rhs => false,
                (Included(lhs), Included(rhs))
                | (Included(lhs), Excluded(rhs))
                | (Excluded(lhs), Included(rhs))
                | (Excluded(lhs), Excluded(rhs)) => lhs.cmp(rhs) != Less,
            },
            Equal => true,
            Greater => false,
        }
    }
    fn touches(&self, other: &Self) -> bool {
        let (left, right) = match self.cmp(&other) {
            Less => (&self, &other),
            Equal => return true,
            Greater => (&other, &self),
        };
        match (&left.1, &right.0) {
            (Unbounded, _) | (_, Unbounded) => true,
            (Included(le), Included(rs))
            | (Included(le), Excluded(rs))
            | (Excluded(le), Included(rs)) => rs <= le,
            (Excluded(le), Excluded(rs)) => rs < le,
        }
    }
}

fn toggle_bound<T: Ord>(b: Bound<T>) -> Bound<T> {
    match b {
        Included(x) => Excluded(x),
        Excluded(x) => Included(x),
        Unbounded => Unbounded,
    }
}

impl<T: Ord> Ord for Interval<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_empty() && other.is_empty() {
            return Equal;
        }
        if &self.0 != &other.0 {
            return match (&self.0, &other.0) {
                (Unbounded, _) => Less,
                (_, Unbounded) => Greater,
                (Included(lhs), Excluded(rhs)) if lhs == rhs => Less,
                (Excluded(lhs), Included(rhs)) if lhs == rhs => Greater,
                (Included(lhs), Included(rhs))
                | (Included(lhs), Excluded(rhs))
                | (Excluded(lhs), Included(rhs))
                | (Excluded(lhs), Excluded(rhs)) => lhs.cmp(rhs),
            };
        }
        if &self.1 != &other.1 {
            return match (&self.1, &other.1) {
                (_, Unbounded) => Less,
                (Unbounded, _) => Greater,
                (Excluded(lhs), Included(rhs)) if lhs == rhs => Less,
                (Included(lhs), Excluded(rhs)) if lhs == rhs => Greater,
                (Included(lhs), Included(rhs))
                | (Included(lhs), Excluded(rhs))
                | (Excluded(lhs), Included(rhs))
                | (Excluded(lhs), Excluded(rhs)) => lhs.cmp(rhs),
            };
        }
        Equal
    }
}

impl<T: Ord> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> PartialEq for Interval<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

/// 区間の集合。
///
/// 区間の追加・削除を行うことができる。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntervalSet<T: Ord> {
    buf: BTreeSet<Interval<T>>,
}

impl<T: Clone + Ord + Debug> IntervalSet<T> {
    /// 空集合で初期化する。
    pub fn new() -> Self {
        Self {
            buf: BTreeSet::new(),
        }
    }

    /// 区間 `r` を追加する。
    pub fn insert<R: RangeBounds<T>>(&mut self, r: R) {
        let mut r: Interval<T> = (r.start_bound(), r.end_bound()).into();
        if r.is_empty() {
            return;
        }
        self.remove_subset(r.clone());
        match self.buf.range(..r.clone()).cloned().next_back() {
            Some(x) if x.is_superset(&r) => return,
            Some(x) if x.touches(&r) => {
                self.buf.remove(&x);
                r.0 = x.0;
            }
            _ => {}
        }
        match self.buf.range(r.clone()..).cloned().next() {
            Some(x) if x.touches(&r) => {
                self.buf.remove(&x);
                r.1 = x.1;
            }
            _ => {}
        }
        self.buf.insert(r);
    }

    /// 区間 `r` を削除する。
    pub fn remove<R: RangeBounds<T>>(&mut self, r: R) {
        let r: Interval<T> = (r.start_bound(), r.end_bound()).into();
        if r.is_empty() {
            return;
        }
        self.remove_subset(r.clone());
        match self.buf.range(..r.clone()).cloned().next_back() {
            Some(x) if x.is_superset(&r) => {
                self.buf.remove(&x);
                let Interval(r0, r1) = r;
                self.buf.insert(Interval(x.0, toggle_bound(r0)));
                self.buf.insert(Interval(toggle_bound(r1), x.1));
                return;
            }
            Some(mut x) if x.touches(&r) => {
                self.buf.remove(&x);
                x.1 = toggle_bound(r.0.clone());
                self.buf.insert(x);
            }
            _ => {}
        }
        match self.buf.range(r.clone()..).cloned().next() {
            Some(mut x) if x.touches(&r) => {
                self.buf.remove(&x);
                x.0 = toggle_bound(r.1.clone());
                self.buf.insert(x);
            }
            _ => {}
        }
    }

    /// 空集合に戻す。
    pub fn clear(&mut self) {
        self.buf.clear();
    }

    /// `x` 以上の値で、集合中の区間に含まれない最小のものを返す。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Included, Excluded, Unbounded};
    /// use nekolib::ds::IntervalSet;
    ///
    /// let mut s = IntervalSet::new();
    /// s.insert(1..5);
    /// s.insert(7..=10);
    /// s.insert(15..);
    ///
    /// assert_eq!(s.mex(0), Included(0));
    /// assert_eq!(s.mex(1), Included(5));
    /// assert_eq!(s.mex(6), Included(6));
    /// assert_eq!(s.mex(7), Excluded(10));
    /// assert_eq!(s.mex(15), Unbounded);
    /// ```
    pub fn mex(&self, x: T) -> Bound<T> {
        if self.buf.is_empty() {
            return Included(x);
        }
        match self
            .buf
            .range(..Interval(Included(x.clone()), Unbounded))
            .next_back()
        {
            Some(Interval(_, Included(y))) if y < &x => Included(x),
            Some(Interval(_, Included(y))) => Excluded(y.clone()),
            Some(Interval(_, Excluded(y))) if y <= &x => Included(x),
            Some(Interval(_, Excluded(y))) => Included(y.clone()),
            Some(Interval(_, Unbounded)) => Unbounded,
            None => Included(x),
        }
    }

    fn remove_subset(&mut self, r: Interval<T>) {
        let rem: Vec<Interval<T>> = match &r {
            Interval(Unbounded, Unbounded) => {
                self.buf.clear();
                return;
            }
            Interval(Included(lo), Unbounded)
            | Interval(Excluded(lo), Unbounded) => self.buf.range((
                Included(Interval(Included(lo.clone()), Included(lo.clone()))),
                Unbounded,
            )),
            Interval(Unbounded, Included(hi))
            | Interval(Unbounded, Excluded(hi)) => self.buf.range((
                Unbounded,
                Included(Interval(Included(hi.clone()), Included(hi.clone()))),
            )),
            Interval(Included(lo), Included(hi))
            | Interval(Included(lo), Excluded(hi))
            | Interval(Excluded(lo), Included(hi))
            | Interval(Excluded(lo), Excluded(hi)) => self.buf.range((
                Included(Interval(Included(lo.clone()), Included(lo.clone()))),
                Included(Interval(Included(hi.clone()), Included(hi.clone()))),
            )),
        }
        .cloned()
        .collect();
        for k in rem.into_iter().filter(|x| r.is_superset(x)) {
            self.buf.remove(&k);
        }
    }
}
