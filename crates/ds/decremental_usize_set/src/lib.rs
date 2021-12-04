//! `usize` の decremental set。

/// `usize` の decremental set。
///
/// $\\{0, 1, \\dots, u-1\\}$ で初期化し、要素を取り除く操作を行える。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(n/w)$|
/// |`remove`|amortized $O(1)$|
/// |`less`|amortized $O(1)$|
/// |`less_equal`|amortized $O(1)$|
/// |`greater`|amortized $O(1)$|
/// |`greater_equal`|amortized $O(1)$|
///
/// 空間：$O(n)$ bits.
///
/// # References
/// - <https://atcoder.jp/contests/abc228/editorial/2962>
/// - <https://twitter.com/noshi91/status/1389116169634795525>
///
/// # Examples
/// ```
/// use nekolib::ds::DecrementalUsizeSet;
///
/// let mut set = DecrementalUsizeSet::new(6);
/// assert_eq!(set.universe_len(), 6);
/// assert_eq!(set.len(), 6);
///
/// set.remove(3);
/// assert_eq!(set.less(3), Some(2));
/// assert_eq!(set.less_equal(3), Some(2));
/// assert_eq!(set.greater(3), Some(4));
/// assert_eq!(set.greater_equal(3), Some(4));
///
/// assert_eq!(set.less(4), Some(2));
/// assert_eq!(set.less_equal(4), Some(4));
/// assert_eq!(set.greater(4), Some(5));
/// assert_eq!(set.greater_equal(4), Some(4));
///
/// assert_eq!(set.less(0), None);
/// assert_eq!(set.greater(5), None);
///
/// set.remove(0);
/// set.remove(5);
/// assert_eq!(set.less_equal(0), None);
/// assert_eq!(set.greater_equal(5), None);
///
/// assert!(set.contains(4));
/// assert!(!set.contains(3));
///
/// assert_eq!(set.universe_len(), 6);
/// assert_eq!(set.len(), 3);
/// ```
pub struct DecrementalUsizeSet {
    u: usize,
    len: usize,
    small: Vec<usize>,
    large: UnionFind,
}

const WORD_SIZE: usize = 0_usize.count_zeros() as usize;

fn bsf(i: usize) -> usize { i.trailing_zeros() as usize }
fn bsr(i: usize) -> usize { WORD_SIZE - 1 - i.leading_zeros() as usize }

impl DecrementalUsizeSet {
    /// $S\\gets\\{0, 1, \\dots, u-1\\}$ で初期化。
    pub fn new(u: usize) -> Self {
        let mut small = vec![!0_usize; u / WORD_SIZE + 1];
        let mut large = UnionFind::new(small.len() + 1);
        let div = u / WORD_SIZE;
        let rem = u % WORD_SIZE;
        small[div] = !(!0_usize << rem);
        if rem == 0 {
            large.unite(div, div + 1);
        }
        Self { u, len: u, small, large }
    }

    /// $u$ を返す。
    pub fn universe_len(&self) -> usize { self.u }
    /// $|S|$ を返す。
    pub fn len(&self) -> usize { self.len }
    /// $S=\\emptyset$ を返す。
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// $i\\in S$ を返す。
    pub fn contains(&self, i: usize) -> bool {
        let div = i / WORD_SIZE;
        let rem = i % WORD_SIZE;
        div < self.small.len() && self.small[div] >> rem & 1 != 0
    }

    /// $\\max\_{j\\lt i}\\text{ s.t. }j\\in S$ を返す。
    pub fn less(&self, i: usize) -> Option<usize> {
        if i == 0 {
            None
        } else {
            self.less_equal(i - 1)
        }
    }
    /// $\\max\_{j\\le i}\\text{ s.t. }j\\in S$ を返す。
    pub fn less_equal(&self, i: usize) -> Option<usize> {
        let i = i.min(self.u - 1) + 1;
        let div = i / WORD_SIZE;
        let rem = i % WORD_SIZE;
        let m = self.small[div] & !(!0_usize << rem);
        if m != 0 {
            return Some(div * WORD_SIZE + bsr(m));
        }
        let b = self.large.left(div);
        if b == 0 {
            None
        } else {
            Some((b - 1) * WORD_SIZE + bsr(self.small[b - 1]))
        }
    }
    /// $\\min\_{j\\gt i}\\text{ s.t. }j\\in S$ を返す。
    pub fn greater(&self, i: usize) -> Option<usize> {
        if i == self.u {
            None
        } else {
            self.greater_equal(i + 1)
        }
    }
    /// $\\min\_{j\\ge i}\\text{ s.t. }j\\in S$ を返す。
    pub fn greater_equal(&self, i: usize) -> Option<usize> {
        let div = i / WORD_SIZE;
        let rem = i % WORD_SIZE;
        if div >= self.small.len() {
            return None;
        }
        let m = self.small[div] & !0_usize << rem;
        if m != 0 {
            return Some(div * WORD_SIZE + bsf(m));
        }
        let b = self.large.right(div + 1);
        if b == self.small.len() {
            None
        } else {
            Some(b * WORD_SIZE + bsf(self.small[b]))
        }
    }

    /// $S\\gets S\\setminus\\{i\\}$ で更新する。
    pub fn remove(&mut self, i: usize) -> bool {
        let div = i / WORD_SIZE;
        let rem = i % WORD_SIZE;
        if div >= self.small.len() || self.small[div] >> rem & 1 == 0 {
            return false;
        }
        self.len -= 1;
        self.small[div] &= !(1_usize << rem);
        if self.small[div] == 0 {
            self.large.unite(div, div + 1);
        }
        true
    }
}

#[derive(Clone, Copy)]
enum Item {
    Parent(usize),
    Size(usize),
}
use Item::{Parent, Size};

use std::cell::RefCell;

struct UnionFind {
    buf: RefCell<Vec<Item>>,
    left: Vec<usize>,
    right: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            buf: RefCell::new(vec![Size(1); n]),
            left: (0..n).collect(),
            right: (0..n).collect(),
        }
    }

    fn repr(&self, mut i: usize) -> usize {
        let mut buf = self.buf.borrow_mut();
        let res = {
            let mut i = i;
            while let Parent(ni) = buf[i] {
                i = ni;
            }
            i
        };
        while let Parent(ni) = buf[i] {
            buf[i] = Parent(res);
            i = ni;
        }
        res
    }

    pub fn left(&self, i: usize) -> usize { self.left[self.repr(i)] }
    pub fn right(&self, i: usize) -> usize { self.right[self.repr(i)] }

    pub fn unite(&mut self, il: usize, ir: usize) {
        let il = self.repr(il);
        let ir = self.repr(ir);
        let mut buf = self.buf.borrow_mut();
        let (sl, sr) = match (buf[il], buf[ir]) {
            (Size(sl), Size(sr)) => (sl, sr),
            _ => unreachable!(),
        };
        if sl < sr {
            buf[ir] = Size(sl + sr);
            buf[il] = Parent(ir);
            self.left[ir] = self.left[il];
        } else {
            buf[il] = Size(sl + sr);
            buf[ir] = Parent(il);
            self.right[il] = self.right[ir];
        }
    }
}
