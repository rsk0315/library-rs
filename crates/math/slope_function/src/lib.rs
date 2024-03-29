//! 区分線形凸関数。

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Bound, RangeInclusive};

/// 区分線形凸関数。
///
/// 整数の多重集合 $L$, $R$ に対して、次の形で表せる関数を管理する：
/// $$ f(x) = c + \\sum\_{l\\in L} (l-x)\_+ + \\sum\_{r\\in R} (x-r)\_+. $$
/// ここで、$(a-x)\_+ = \\max\\{0, a-x\\}$、$(x-a)\_+ = \\max\\{0, x-a\\}$ とする。
/// たとえば、$|x-a| = (a-x)\_+ + (x-a)\_+$ と書ける。
///
/// # Idea
/// `todo!()`
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$O(1)$|
/// |`add_const`|$O(1)$|
/// |`add_left`, `add_right`, `add_abs`|$O(\\log(\|L\|) + \\log(\|R\|))$|
/// |`min_left`, `min_right`|amortized $O(1)$|
/// |`shift`, `window`|$O(1)$|
/// |`min`, `argmin`|$O(1)$|
///
/// # Examples
/// ```
/// use std::ops::Bound::{Included, Unbounded};
///
/// use nekolib::math::SlopeFunction;
///
/// let mut sf = SlopeFunction::new();
/// sf.add_left(-1);
/// sf.add_left(3);
/// sf.add_right(2);
/// sf.add_const(-10);
/// // f(x) = 0.max(-1-x) + 0.max(3-x) + 0.max(x-2) - 10
/// //   x  | -5 -4 -3 -2 -1  0  1  2  3  4  5
/// // f(x) |  2  0 -2 -4 -6 -7 -8 -9 -9 -8 -7
/// assert_eq!(sf.min(), -9);
/// assert_eq!(sf.argmin(), (Included(2), Included(3)));
/// ```
///
/// # Notes
/// $(a\_1, a\_2, \\dots, a\_n)$ の中央値を $a\_{\\text{med}}$ とすると、
/// $\\sum\_{i=1}^n |x-a\_i|$ は $x = a\_{\\text{med}}$ のとき最小となる。
/// このことから、値の追加と中央値を求めるクエリを処理できる。
///
/// ```
/// use std::ops::Bound::{Included, Excluded, Unbounded};
///
/// use nekolib::math::SlopeFunction;
///
/// #[derive(Clone, Default)]
/// struct IncrementalMedian(SlopeFunction<i32>);
///
/// impl IncrementalMedian {
///     fn new() -> Self { Self::default() }
///     fn insert(&mut self, a: i32) { self.0.add_abs(a); }
///     fn median(&self) -> Option<i32> {
///         match self.0.argmin().0 {
///             Included(x) => Some(x),
///             Excluded(_) => unreachable!(),
///             Unbounded => None,
///         }
///     }
/// }
///
/// let mut im = IncrementalMedian::new();
/// assert_eq!(im.median(), None); // {{}}
/// im.insert(2);
/// assert_eq!(im.median(), Some(2)); // {{2}}
/// im.insert(3);
/// assert_eq!(im.median(), Some(2)); // {{2, 3}}
/// im.insert(1);
/// assert_eq!(im.median(), Some(2)); // {{1, 2, 3}}
/// im.insert(1);
/// assert_eq!(im.median(), Some(1)); // {{1, 1, 2, 3}}
/// ```
///
/// # References
/// - <https://maspypy.com/slope-trick-1-%E8%A7%A3%E8%AA%AC%E7%B7%A8>
#[derive(Clone, Debug, Default)]
pub struct SlopeFunction<I: Ord> {
    left: BinaryHeap<I>,
    right: BinaryHeap<Reverse<I>>,
    min: I,
    shl: I,
    shr: I,
}

impl<I: SlopeTrickInt> SlopeFunction<I> {
    /// $f(x) = 0$ で初期化する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::Unbounded;
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let sf = SlopeFunction::<i32>::new();
    /// assert_eq!(sf.min(), 0);
    /// assert_eq!(sf.argmin(), (Unbounded, Unbounded));
    /// ```
    pub fn new() -> Self { Self::default() }
    /// $f(x) \\xleftarrow{+} c$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// assert_eq!(sf.min(), 0);
    /// sf.add_const(3);
    /// assert_eq!(sf.min(), 3);
    /// sf.add_const(-1);
    /// assert_eq!(sf.min(), 2);
    /// ```
    pub fn add_const(&mut self, c: I) { self.min += c; }
    /// $f(x) \\xleftarrow{+} (l-x)\_+$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Included, Unbounded};
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_left(4);
    /// assert_eq!(sf.argmin(), (Included(4), Unbounded));
    /// ```
    pub fn add_left(&mut self, l: I) {
        if self.right.is_empty() {
            self.left.push(l);
            return;
        }
        self.min += l.doz(self.right.peek().unwrap().0);
        self.right.push(Reverse(l));
        let l = self.right.pop().unwrap().0;
        self.left.push(l);
    }
    /// $f(x) \\xleftarrow{+} (x-r)\_+$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Included, Unbounded};
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_right(4);
    /// assert_eq!(sf.argmin(), (Unbounded, Included(4)));
    /// ```
    pub fn add_right(&mut self, r: I) {
        if self.left.is_empty() {
            self.right.push(Reverse(r));
            return;
        }
        self.min += self.left.peek().unwrap().doz(r);
        self.left.push(r);
        let r = self.left.pop().unwrap();
        self.right.push(Reverse(r));
    }
    /// $f(x) \\xleftarrow{+} |x-a|$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::Included;
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// assert_eq!(sf.argmin(), (Included(4), Included(4)));
    /// ```
    pub fn add_abs(&mut self, a: I) {
        self.add_left(a);
        self.add_right(a);
    }
    /// $g(x) = \\min\_{y\\le x} f(y)$ として、$f\\gets g$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Included, Unbounded};
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// assert_eq!(sf.argmin(), (Included(4), Included(4)));
    /// sf.min_left();
    /// assert_eq!(sf.argmin(), (Included(4), Unbounded));
    /// ```
    pub fn min_left(&mut self) { self.right.clear(); }
    /// $g(x) = \\min\_{y\\ge x} f(y)$ として、$f\\gets g$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::{Included, Unbounded};
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// assert_eq!(sf.argmin(), (Included(4), Included(4)));
    /// sf.min_right();
    /// assert_eq!(sf.argmin(), (Unbounded, Included(4)));
    /// ```
    pub fn min_right(&mut self) { self.left.clear(); }
    /// $g(x) = f(x-a)$ として、$f\\gets g$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::Included;
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// assert_eq!(sf.argmin(), (Included(4), Included(4)));
    /// sf.shift(2);
    /// assert_eq!(sf.argmin(), (Included(6), Included(6)));
    /// ```
    pub fn shift(&mut self, s: I) {
        self.shl += s;
        self.shr += s;
    }
    /// $[a, b]$ に対して $g(x) = \\min\_{y\\in[x-b, x-a]} f(y)$ として、$f\\gets g$ で更新する。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::Included;
    ///
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// assert_eq!(sf.argmin(), (Included(4), Included(4)));
    /// sf.window(-1..=2);
    /// assert_eq!(sf.argmin(), (Included(3), Included(6)));
    /// ```
    pub fn window(&mut self, window: RangeInclusive<I>) {
        self.shl += *window.start();
        self.shr += *window.end();
    }
    /// $\\min\_{x\\in\\mathbb{R}} f(x)$ を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// sf.add_const(1);
    /// assert_eq!(sf.min(), 1);
    /// ```
    pub fn min(&self) -> I { self.min }
    /// $\\argmin\_{x\\in\\mathbb{R}} f(x)$ を返す。
    ///
    /// # Examples
    /// ```
    /// use std::ops::Bound::Included;
    /// use nekolib::math::SlopeFunction;
    ///
    /// let mut sf = SlopeFunction::new();
    /// sf.add_abs(4);
    /// sf.add_const(1);
    /// assert_eq!(sf.argmin(), (Included(4), Included(4)));
    /// ```
    pub fn argmin(&self) -> (Bound<I>, Bound<I>) {
        let left = match self.left.peek() {
            Some(&x) => Bound::Included(x + self.shl),
            None => Bound::Unbounded,
        };
        let right = match self.right.peek() {
            Some(&Reverse(x)) => Bound::Included(x + self.shr),
            None => Bound::Unbounded,
        };
        (left, right)
    }
}

pub trait SlopeTrickInt:
    Copy + Add<Output = Self> + AddAssign + Default + Ord
{
    // unsigned でいうところの saturating_sub
    fn doz(self, rhs: Self) -> Self;
}

macro_rules! impl_slope_trick_int {
    ( $($ty:tt)* ) => { $(
        impl SlopeTrickInt for $ty {
            fn doz(self, rhs: Self) -> Self {
                0.max(self - rhs)
            }
        }
    )* }
}

impl_slope_trick_int! { i8 i16 i32 i64 i128 isize }
