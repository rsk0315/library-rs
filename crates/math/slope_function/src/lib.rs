//! 区分線形凸関数。

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::{Bound, RangeInclusive};

/// 区分線形凸関数。
///
/// 次の形で表せる関数を管理する：
/// $$ f(x) = c + \\sum\_{l\\in L} (l-x)\_+ + \\sum\_{r\\in R} (x-r)\_+. $$
/// ここで、$(a-x)\_+ = \\max\\{0, a-x\\}$、$(x-a)\_+ = \\max\\{0, x-a\\}$ とする。
/// たとえば、$|x-a| = (a-x)\_+ + (x-a)\_+$ と書ける。
///
/// # References
/// - <https://maspypy.com/slope-trick-1-%E8%A7%A3%E8%AA%AC%E7%B7%A8>
#[derive(Clone, Default)]
pub struct SlopeFunction {
    left: BinaryHeap<i128>,
    right: BinaryHeap<Reverse<i128>>,
    min: i128,
    shl: i128,
    shr: i128,
}

impl SlopeFunction {
    /// $f(x) = 0$ で初期化する。
    pub fn new() -> Self { Self::default() }
    /// $f(x) \\xleftarrow{+} c$ で更新する。
    pub fn add_const(&mut self, c: i128) { self.min += c; }
    /// $f(x) \\xleftarrow{+} (x-r)\_+$ で更新する。
    pub fn add_right(&mut self, r: i128) {
        if self.left.is_empty() {
            self.right.push(Reverse(r));
            return;
        }
        self.min += 0.max(*self.left.peek().unwrap() - r);
        self.left.push(r);
        let r = self.left.pop().unwrap();
        self.right.push(Reverse(r));
    }
    /// $f(x) \\xleftarrow{+} (l-x)\_+$ で更新する。
    pub fn add_left(&mut self, l: i128) {
        if self.right.is_empty() {
            self.left.push(l);
            return;
        }
        self.min += 0.max(l - self.right.peek().unwrap().0);
        self.right.push(Reverse(l));
        let l = self.right.pop().unwrap().0;
        self.left.push(l);
    }
    /// $f(x) \\xleftarrow{+} |x-a|$ で更新する。
    pub fn add_abs(&mut self, a: i128) {
        self.add_left(a);
        self.add_right(a);
    }
    /// $g(x) = \\min\_{y\\le x} f(y)$ として、$f\\gets g$ で更新する。
    pub fn min_left(&mut self) { self.right.clear(); }
    /// $g(x) = \\min\_{y\\ge x} f(y)$ として、$f\\gets g$ で更新する。
    pub fn min_right(&mut self) { self.left.clear(); }
    /// $g(x) = f(x-a)$ として、$f\\gets g$ で更新する。
    pub fn shift(&mut self, s: i128) {
        self.shl += s;
        self.shr += s;
    }
    /// $g(x) = \\min\_{y\\in[x-b, x-a]} f(y)$ として、$f\\gets g$ で更新する。
    pub fn window(&mut self, window: RangeInclusive<i128>) {
        self.shl += *window.start();
        self.shr += *window.end();
    }
    /// $\\min\_{x\\in\\mathbb{R}} f(x)$ を返す。
    pub fn min(&self) -> i128 { self.min }
    /// $\\argmin\_{x\\in\\mathbb{R}} f(x)$ を返す。
    pub fn argmin(&self) -> (Bound<i128>, Bound<i128>) {
        let left = match self.left.peek() {
            Some(&x) => Bound::Included(x),
            None => Bound::Unbounded,
        };
        let right = match self.right.peek() {
            Some(&Reverse(x)) => Bound::Included(x),
            None => Bound::Unbounded,
        };
        (left, right)
    }
}
