//! bit set。

use std::ops::{Range, RangeBounds};

use buf_range::{bounds_within, check_bounds};

/// bit set。
///
/// # Examples
/// ```
/// use nekolib::ds::BitSet;
///
/// let mut bs = BitSet::new(10);
/// bs.insert(3);
/// bs.insert(6);
/// bs.insert(7);
///
/// assert_eq!(format!("{}", bs), "0001001100");
/// assert_eq!(format!("{:?}", bs), "{3, 6, 7}");
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct BitSet {
    buf: Vec<u128>,
    cap: usize,
    len: usize,
}

const WORD_SIZE: usize = 128;

impl BitSet {
    /// 初期化。
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let bs = BitSet::new(10);
    /// ```
    pub fn new(cap: usize) -> Self {
        let buf = vec![0; (cap + WORD_SIZE - 1) / WORD_SIZE];
        Self { buf, cap, len: 0 }
    }

    /// $i$ を含むとき真を返す。
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    ///
    /// assert!(!bs.contains(0));
    /// assert!( bs.contains(3));
    /// ```
    pub fn contains(&self, i: usize) -> bool {
        check_bounds(i, self.cap);
        let (large, small) = (i / WORD_SIZE, i % WORD_SIZE);
        self.buf[large] >> small & 1 != 0
    }

    /// 区間に含まれる個数を返す。
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    /// bs.insert(5);
    ///
    /// assert_eq!(bs.count(0..5), 1);
    /// assert_eq!(bs.count(..), 2);
    /// ```
    pub fn count(&self, range: impl RangeBounds<usize>) -> usize {
        self.range_word(range).map(|w| w.count_ones() as usize).sum()
    }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// $i$ を追加する。
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    ///
    /// assert!(!bs.contains(3));
    /// bs.insert(3);
    /// assert!( bs.contains(3));
    /// ```
    pub fn insert(&mut self, i: usize) {
        check_bounds(i, self.cap);
        let (large, small) = (i / WORD_SIZE, i % WORD_SIZE);
        if self.buf[large] >> small & 1 == 0 {
            self.buf[large] |= 1 << small;
            self.len += 1;
        }
    }

    /// $i$ を削除する。
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    ///
    /// assert!(!bs.contains(3));
    /// bs.insert(3);
    /// assert!( bs.contains(3));
    /// bs.remove(3);
    /// assert!(!bs.contains(3));
    /// ```
    pub fn remove(&mut self, i: usize) {
        check_bounds(i, self.cap);
        let (large, small) = (i / WORD_SIZE, i % WORD_SIZE);
        if self.buf[large] >> small & 1 != 0 {
            self.buf[large] &= !(1 << small);
            self.len -= 1;
        }
    }

    /// 1 の添字を返す。
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    /// bs.insert(5);
    ///
    /// assert_eq!(bs.iter().collect::<Vec<_>>(), [3, 5]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        let first = if self.cap == 0 { None } else { self.min_ge(0) };
        std::iter::successors(first, move |&i| self.min_gt(i))
    }

    pub fn range_index(&self, range: impl RangeBounds<usize>) -> RangeIndex {
        let Range { start, end } = bounds_within(range, self.cap);
        RangeIndex::new(start, end, self)
    }

    pub fn range_word(&self, range: impl RangeBounds<usize>) -> RangeWord {
        let Range { start, end } = bounds_within(range, self.cap);
        check_bounds(end, self.cap + 1);
        RangeWord::new(start, end, &self)
    }

    /// `(0..i).rev().find(|&i| it.contains(i))`
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    /// bs.insert(5);
    ///
    /// assert_eq!(bs.max_lt(2), None);
    /// assert_eq!(bs.max_lt(3), None);
    /// assert_eq!(bs.max_lt(4), Some(3));
    /// assert_eq!(bs.max_lt(5), Some(3));
    /// assert_eq!(bs.max_lt(6), Some(5));
    /// ```
    pub fn max_lt(&self, i: usize) -> Option<usize> {
        self.range_word(..i)
            .zip(0..(i + WORD_SIZE - 1) / WORD_SIZE)
            .rev()
            .find(|&(w, _)| w != 0)
            .map(|(w, i)| {
                i * WORD_SIZE + (WORD_SIZE - 1 - w.leading_zeros() as usize)
            })
    }

    /// `(0..=i).rev().find(|&i| it.contains(i))`
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    /// bs.insert(5);
    ///
    /// assert_eq!(bs.max_le(2), None);
    /// assert_eq!(bs.max_le(3), Some(3));
    /// assert_eq!(bs.max_le(4), Some(3));
    /// assert_eq!(bs.max_le(5), Some(5));
    /// assert_eq!(bs.max_le(6), Some(5));
    /// ```
    pub fn max_le(&self, i: usize) -> Option<usize> {
        self.range_word(..=i)
            .zip(0..i / WORD_SIZE + 1)
            .rev()
            .find(|&(w, _)| w != 0)
            .map(|(w, i)| {
                i * WORD_SIZE + (WORD_SIZE - 1 - w.leading_zeros() as usize)
            })
    }

    /// `(i + 1..n).rev().find(|&i| it.contains(i))`
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    /// bs.insert(5);
    ///
    /// assert_eq!(bs.min_gt(2), Some(3));
    /// assert_eq!(bs.min_gt(3), Some(5));
    /// assert_eq!(bs.min_gt(4), Some(5));
    /// assert_eq!(bs.min_gt(5), None);
    /// assert_eq!(bs.min_gt(6), None);
    /// ```
    pub fn min_gt(&self, i: usize) -> Option<usize> {
        self.range_word(i + 1..self.cap)
            .zip((i + 1) / WORD_SIZE..(self.cap + WORD_SIZE - 1) / WORD_SIZE)
            .find(|&(w, _)| w != 0)
            .map(|(w, i)| i * WORD_SIZE + (w.trailing_zeros() as usize))
    }

    /// `(i..n).rev().find(|&i| it.contains(i))`
    /// # Examples
    /// ```
    /// use nekolib::ds::BitSet;
    ///
    /// let mut bs = BitSet::new(10);
    /// bs.insert(3);
    /// bs.insert(5);
    ///
    /// assert_eq!(bs.min_ge(2), Some(3));
    /// assert_eq!(bs.min_ge(3), Some(3));
    /// assert_eq!(bs.min_ge(4), Some(5));
    /// assert_eq!(bs.min_ge(5), Some(5));
    /// assert_eq!(bs.min_ge(6), None);
    /// ```
    pub fn min_ge(&self, i: usize) -> Option<usize> {
        self.range_word(i..self.cap)
            .zip(i / WORD_SIZE..(self.cap + WORD_SIZE - 1) / WORD_SIZE)
            .find(|&(w, _)| w != 0)
            .map(|(w, i)| i * WORD_SIZE + (w.trailing_zeros() as usize))
    }
}

impl Extend<usize> for BitSet {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}

pub struct RangeWord<'a> {
    start: usize,
    end: usize,
    bs: &'a BitSet,
}

impl<'a> RangeWord<'a> {
    pub fn new(start: usize, end: usize, bs: &'a BitSet) -> Self {
        Self { start, end, bs }
    }
}

impl Iterator for RangeWord<'_> {
    type Item = u128;
    fn next(&mut self) -> Option<u128> {
        if self.start >= self.end {
            return None;
        }
        let w = self.bs.buf[self.start / WORD_SIZE];
        let mut mask = !0 << (self.start % WORD_SIZE);
        if self.start / WORD_SIZE == self.end / WORD_SIZE {
            mask &= !(!0 << (self.end % WORD_SIZE));
        }
        self.start = (self.start / WORD_SIZE + 1) * WORD_SIZE;
        Some(w & mask)
    }
}

impl DoubleEndedIterator for RangeWord<'_> {
    fn next_back(&mut self) -> Option<u128> {
        if self.start >= self.end {
            return None;
        }
        let w = self.bs.buf[(self.end - 1) / WORD_SIZE];
        let mut mask = !0;
        if self.end % WORD_SIZE != 0 {
            mask = !(!0 << (self.end % WORD_SIZE));
        }
        if self.start / WORD_SIZE == self.end / WORD_SIZE {
            mask &= !(!0 << (self.end % WORD_SIZE));
        }
        self.end = (self.end - 1) / WORD_SIZE * WORD_SIZE;
        Some(w & mask)
    }
}

impl ExactSizeIterator for RangeWord<'_> {
    fn len(&self) -> usize {
        if self.start == self.end {
            0
        } else {
            (self.end + WORD_SIZE - 1) / WORD_SIZE - self.start / WORD_SIZE
        }
    }
}

pub struct RangeIndex<'a> {
    start: usize,
    end: usize,
    bs: &'a BitSet,
}

impl<'a> RangeIndex<'a> {
    pub fn new(start: usize, end: usize, bs: &'a BitSet) -> Self {
        Self { start, end, bs }
    }
}

impl Iterator for RangeIndex<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.start >= self.end {
            return None;
        }
        match self.bs.min_ge(self.start) {
            Some(i) if i < self.end => {
                self.start = i + 1;
                Some(i)
            }
            _ => {
                self.start = self.end;
                None
            }
        }
    }
}

impl DoubleEndedIterator for RangeIndex<'_> {
    fn next_back(&mut self) -> Option<usize> {
        if self.start >= self.end {
            return None;
        }
        match self.bs.max_le(self.end) {
            Some(0) => {
                self.end = 0;
                Some(0)
            }
            Some(i) if i >= self.start => {
                self.end = i - 1;
                Some(i)
            }
            _ => {
                self.end = self.start;
                None
            }
        }
    }
}

use std::fmt;

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (w, i) in self.range_word(..).zip((0..).step_by(WORD_SIZE)) {
            let mut w = w.reverse_bits();
            let width = if i + WORD_SIZE <= self.cap {
                WORD_SIZE
            } else {
                self.cap % WORD_SIZE
            };
            w >>= WORD_SIZE - width;
            write!(f, "{0:01$b}", w, width)?;
        }
        Ok(())
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(self.range_index(..)).finish()
    }
}

#[test]
fn test_count() {
    use std::collections::BTreeSet;

    let cap = 313;
    let it = std::iter::successors(Some(1_usize), |x| Some(15 * x % cap));
    let len = 40;
    let naive: BTreeSet<_> = it.take(len).collect();

    let mut bs = BitSet::new(cap);
    bs.extend(naive.iter().copied());

    for start in 0..=cap {
        for end in start..=cap {
            let expected = naive.range(start..end).count();
            let actual = bs.count(start..end);
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn test_iter() {
    use std::collections::BTreeSet;

    let cap = 46337;
    let it = std::iter::successors(Some(1_usize), |x| Some(3 * x % cap));
    let len = 2400;
    let naive: BTreeSet<_> = it.take(len).collect();

    let mut bs = BitSet::new(cap);
    bs.extend(naive.iter().copied());

    for i in 0..cap {
        assert_eq!(bs.max_lt(i), naive.range(..i).next_back().copied());
        assert_eq!(bs.max_le(i), naive.range(..=i).next_back().copied());
        assert_eq!(bs.min_ge(i), naive.range(i..).next().copied());
        assert_eq!(bs.min_gt(i), naive.range(i + 1..).next().copied());
    }
}
