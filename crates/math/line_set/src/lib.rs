//! 直線の集合。

use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt::{self, Debug};

type Line = (i128, i128);
type Interval = (i128, i128);

const MIN: i128 = std::i128::MIN;
const MAX: i128 = std::i128::MAX;

/// 直線の集合。
///
/// 以下のクエリを処理する。
/// - 集合 $S = \\emptyset$ で初期化する。
/// - 集合 $S$ に直線 $y = ax+b$ を追加する。
/// - 集合 $S$ 中の直線における、$x=x\_0$ での最小の $y$ 座標を返す。
///
/// # Idea
/// 次の二つの連想配列を管理する。
/// - 直線を与えると、その直線での $y$ 座標が集合中で最小となる区間を返す。
/// - 区間を与えると、その区間において $y$ 座標が最小となる直線を返す。
///
/// `todo!()`
/// - 直線を追加する際に行うことを書く。
/// - 直線が不要かどうかの判定について書く。
///
/// # Applications
/// `todo!()`
/// - いつもの DP に関するメモを書く。
/// - <https://codeforces.com/contest/660/problem/F>
#[derive(Clone, Default)]
pub struct LineSet {
    line_interval: BTreeMap<Reverse<Line>, Interval>,
    interval_line: BTreeMap<Interval, Line>,
}

fn div_both(a: i128, b: i128) -> (i128, i128) {
    let (a, b) = if b < 0 { (-a, -b) } else { (a, b) };
    let div = a / b;
    let rem = a % b;
    let res = if rem < 0 {
        (div - 1, div)
    } else if rem > 0 {
        (div, div + 1)
    } else {
        (div, div)
    };
    res
}

/// 交点の $x$ 座標。
///
/// $y=a\_0x+b\_0$ と $y=a\_1x+b\_1$ ($a\_0 \\gt a\_1$) の交点を求める。
/// 交点の $x$ 座標 $x$ に対して $(\\lfloor x\\rfloor, \\lceil x\\rceil)$ を返す。
fn cross_x((a0, b0): Line, (a1, b1): Line) -> (i128, i128) {
    // a0 * x + b0 = a1 * x + b1
    // (a0-a1) * x = (b1-b0)
    div_both(b1 - b0, a0 - a1)
}

/// 直線の上下判定。
///
/// $y=ax+b$ が $y=a\_lx+b\_l$ と $y=a\_rx+b\_r$ ($a\_l \\gt a\_r$)
/// よりも常に真に上にあれば `true` を返す。
fn is_above((a, b): Line, (al, bl): Line, (ar, br): Line) -> bool {
    (br - b) * (al - a) <= (b - bl) * (a - ar)
}

impl LineSet {
    /// $S = \\emptyset$ で初期化する。
    pub fn new() -> Self { Self::default() }
    /// $S \\xleftarrow{\\cup} ax+b$ で更新する。
    pub fn add_line(&mut self, a: i128, b: i128) {
        if self.line_interval.is_empty() {
            self.line_interval.insert(Reverse((a, b)), (MIN, MAX));
            self.interval_line.insert((MIN, MAX), (a, b));
            return;
        }

        if !self.preinsert(a, b) {
            return;
        }

        self.remove_left(a, b);
        self.remove_right(a, b);
        self.insert(a, b);
    }
    /// 直線 $y=ax+b$ を入れるための前処理。
    ///
    /// すでに傾きが $a$ の直線が入っていて切片が $b$ より大きければそれを取り除く。
    /// $y=ax+b$ を入れる必要があれば `true` を返す。
    fn preinsert(&mut self, a: i128, b: i128) -> bool {
        if let Some((&Reverse((a0, b0)), &(xl, xr))) =
            self.line_interval.range((Reverse((a, MAX)))..).next()
        {
            if a0 == a {
                if b0 <= b {
                    return false;
                }
                self.line_interval.remove(&Reverse((a0, b0)));
                self.interval_line.remove(&(xl, xr));
                return true;
            }
        }
        let left = self.line_interval.range(..Reverse((a, MAX))).next_back();
        let right = self.line_interval.range(Reverse((a, MAX))..).next();
        if let (Some((&Reverse((al, bl)), _)), Some((&Reverse((ar, br)), _))) =
            (left, right)
        {
            if is_above((a, b), (al, bl), (ar, br)) {
                return false;
            }
        }
        true
    }
    fn insert(&mut self, a: i128, b: i128) {
        // interval が [x, x] になる要素が生じるケースが心配
        // 格子点で交わる場合でも重複しないようにしておけば心配ないかも？
        let left = self.line_interval.range(..Reverse((a, MAX))).next_back();
        let xl = match left {
            Some((&Reverse(line0), &(xl0, xr0))) => {
                let (xr, xl) = cross_x((a, b), line0);
                self.interval_line.remove(&(xl0, xr0));
                if xl0 < xr || (xl0 == xr && xl0 < xl) {
                    self.line_interval.get_mut(&Reverse(line0)).unwrap().1 = xr;
                    self.interval_line.insert((xl0, xr), line0);
                } else {
                    self.line_interval.remove(&Reverse(line0));
                }
                xl
            }
            None => MIN,
        };
        let right = self.line_interval.range(Reverse((a, MAX))..).next();
        let xr = match right {
            Some((&Reverse(line0), &(xl0, xr0))) => {
                let (xr, xl) = cross_x((a, b), line0);
                self.interval_line.remove(&(xl0, xr0));
                if xl < xr0 || (xl == xr0 && xr < xr0) {
                    self.line_interval.get_mut(&Reverse(line0)).unwrap().0 = xl;
                    self.interval_line.insert((xl, xr0), line0);
                } else {
                    self.line_interval.remove(&Reverse(line0));
                }
                xr
            }
            None => MAX,
        };
        self.line_interval.insert(Reverse((a, b)), (xl, xr));
        self.interval_line.insert((xl, xr), (a, b));
    }
    fn remove_left(&mut self, a: i128, b: i128) {
        let crit = Reverse((a, MAX));
        let mut rm = vec![];
        for (y0, y1) in self
            .line_interval
            .range(..crit)
            .rev()
            .zip(self.line_interval.range(..crit).rev().skip(1))
        {
            let (&Reverse((a0, b0)), &(xl0, xr0)) = y0;
            let (&Reverse((a1, b1)), _) = y1;
            if is_above((a0, b0), (a1, b1), (a, b)) {
                rm.push(((a0, b0), (xl0, xr0)));
                continue;
            }
            break;
        }
        self.remove(rm);
    }
    fn remove_right(&mut self, a: i128, b: i128) {
        let crit = Reverse((a, MAX));
        let mut rm = vec![];
        for (y0, y1) in self
            .line_interval
            .range(crit..)
            .zip(self.line_interval.range(crit..).skip(1))
        {
            let (&Reverse((a0, b0)), &(xl0, xr0)) = y0;
            let (&Reverse((a1, b1)), _) = y1;
            if is_above((a0, b0), (a, b), (a1, b1)) {
                rm.push(((a0, b0), (xl0, xr0)));
                continue;
            }
            break;
        }
        self.remove(rm);
    }
    fn remove(&mut self, rm: Vec<(Line, Interval)>) {
        for ((a, b), (xl, xr)) in rm {
            self.line_interval.remove(&Reverse((a, b)));
            self.interval_line.remove(&(xl, xr));
        }
    }
    /// $\\min\_{f(x)\\in S} f(x\_0)$ を返す。
    pub fn min_at_point(&self, x0: i128) -> Option<i128> {
        if self.line_interval.is_empty() {
            return None;
        }
        let crit = (x0, x0);
        let (xl, xr) = match self.interval_line.range(crit..).next() {
            Some((&(xl, xr), _)) if xl <= x0 => (xl, xr),
            _ => *self.interval_line.range(..crit).next_back().unwrap().0,
        };
        let (a, b) = self.interval_line[&(xl, xr)];
        Some(a * x0 + b)
    }
}

struct LineDbg(i128, i128);
impl Debug for LineDbg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("y={}x{:+}", self.0, self.1))
    }
}
impl Debug for LineSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(
                self.line_interval.iter().map(
                    |(&Reverse((a, b)), (xl, xr))| (LineDbg(a, b), xl..=xr),
                ),
            )
            .finish()
    }
}

#[test]
fn test_simple() {
    let mut ls = LineSet::new();
    eprintln!("{:?}", ls);
    assert_eq!(ls.min_at_point(1), None);

    let mut f =
        std::iter::successors(Some(185), |&x| Some((x * 291 + 748) % 93739))
            .map(|x| x % 300 - 150);

    let mut naive = vec![];
    for _ in 0..10000 {
        let a = f.next().unwrap();
        let b = f.next().unwrap();
        eprintln!("adding: y={}x{:+}", a, b);
        ls.add_line(a, b);
        naive.push((a, b));
        eprintln!("{:?}", (ls.line_interval.len(), ls.interval_line.len()));
        for x in -100..=100 {
            let expected = naive.iter().map(|&(a, b)| a * x + b).min();
            let got = ls.min_at_point(x);
            if got != expected {
                eprintln!("x: {}", x);
            }
            assert_eq!(got, expected);
        }
    }
}

#[test]
fn test_cross() {
    // 一点でたくさんの直線が交差する場合のテストを書く
}

#[test]
fn test_frac() {
    // ある直線が最小となる区間が格子点を含まない場合のテストを書く
}
