//! 直線の集合。

use std::collections::BTreeMap;
use std::fmt::{self, Debug};

use btree_bimap::BTreeBimap;

/// 直線の集合。
///
/// 以下のクエリを処理する。
/// - 集合 $S \\gets \\emptyset$ で初期化する。
/// - 集合 $S$ に 1 次関数 $\\lambda x.\\; ax+b$ を追加する。
/// - 集合 $S$ 中の関数における、$x=x\_0$ での最小値を返す。
///
/// 言い換えると、直線の追加クエリと、特定の $x$ 座標での $y$
/// 座標の最小値を求めるクエリを捌く。いわゆる CHT。
///
/// # Idea
/// 次の二つの連想配列を管理する。
/// - $a$ を与えると、$\\lambda x.\\; ax+b \\in S$ なる $b$ を返す。
/// - $a$ を与えると、$\\lambda x.\\; ax+b \\in S$ が最小となる $x$ の最大値 $x\_a$ を返す。
///     - こちらは双方向で管理しておく。すなわち、$x\_a\\mapsto a$ の連想配列も持つ。
///
/// 保持しておく必要がある直線を対応する区間の昇順に並べると、傾きの降順に並ぶことに気づく。
/// そこで、追加したい直線の傾きより小さい最大の傾きの直線と、大きい最小の直線と比較し、
/// 新しい直線が必要かどうかをまず確かめる。
/// それが必要なら、追加する直線に近い方から順にすでにある直線を見ていき、
/// 必要なものが見つかるまで削除する。
///
/// クエリを整数とすると、以下が成り立つ。
///
/// $$ \\begin{aligned}
/// f(\\lambda x.\\; a\_l x+b\_l, \\lambda x.\\; a\_r x+b\_r)
/// &= \\max\\,\\{k \\mid a\_l k+b\_l \\le a\_r k+b\_r \\} \\\\
/// &= \\left\\lfloor\\frac{b\_r-b\_l}{a\_l-a\_r}\\right\\rfloor.
/// \\end{aligned} $$
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$O(1)$|
/// |`push`|$O(\\log(\|S\'\|))$|
/// |`min`|$O(\\log(\|S\'\|))$|
///
/// ここで、$S\'$ は $S$ から必要のない直線を除いたものからなる集合である。
///
/// # Applications
/// 次の形式の DP の高速化に使える。
/// $$ \\mathrm{dp}\[i\] = \\min\_{0\\le j\\lt i} (p(j)+q(j)\\cdot r(i)) +s(i). $$
/// $\\min\_{0\\le j\\lt i} (\\bullet)$ の部分が、直線 $y=q(j)\\cdot x+p(j)$ の $x=r(i)$
/// における最小値に相当するためである。$\\mathrm{dp}\[i\]$ の値を求めた後、直線
/// $y=q(i)\\cdot x+p(i)$ を追加していけばよい。ここで、$p(j)$ や $q(j)$ は
/// $\\mathrm{dp}\[j\]$ を含んでもよいし含まなくてもよい。どちらにも $\\mathrm{dp}\[j\]$
/// が含まれない場合には、特に DP 配列のようなものを用意する必要はない。
///
/// たとえば、次のようなものが当てはまる。
/// $$ \\begin{aligned}
/// \\mathrm{dp}\[i\] &= \\min\_{0\\le j\\lt i} (\\mathrm{dp}\[j\]+(a\_j-a\_i)^2) \\\\
/// &= \\min\_{0\\le j\\lt i} ((\\mathrm{dp}\[j\]+a\_j^2) + (-2\\cdot a\_j)\\cdot a\_i)+a\_i^2.
/// \\end{aligned} $$
///
/// お気に入りの例として、[次のような問題](https://codeforces.com/contest/660/problem/F)
/// も解ける：
/// > 整数列 $a = (a\_0, a\_1, \\dots, a\_{n-1})$ が与えられる。
/// > これの空でない区間 $(a\_l, a\_{l+1}, \\dots, a\_{r-1})$
/// > に対し、次の値を考える。
/// > $$ \\sum\_{i=l}^{r-1} (i-l+1)\\cdot a\_i
/// > = 1\\cdot a\_l+2\\cdot a\_{l+1} + \\dots + (r-l)\\cdot a\_{r-1}. $$
/// > 全ての区間の選び方におけるこの値の最大値を求めよ。
/// >
/// > ### Sample
/// > ```text
/// > [5, -1000, 1, -3, 7, -8]
/// >           [ ------ ] => 1 * 1 + (-3) * 2 + 7 * 3 = 16
/// > ```
///
/// $\\sigma(r) = \\sum\_{i=0}^{r-1} a(i)$、$\\tau(r) = \\sum\_{i=0}^{r-1} i\\cdot a(i)$
/// とおくと、次のように変形できる。
/// $$ \\begin{aligned} \\sum\_{i=l}^{r-1} (i-l+1)\\cdot a\_i &=
/// \\sum\_{i=l}^{r-1} i\\cdot a\_i - \\sum\_{i=l}^{r-1} (l-1)\\cdot a\_i \\\\
/// &= (\\tau(r)-\\tau(l)) - (l-1)\\cdot (\\sigma(r) - \\sigma(l))
/// . \\end{aligned} $$
///
/// 右端 $r$ を固定したときの最小値を $\\mathrm{dp}\[r\]$ とおくと、
/// $$ \\begin{aligned} \\mathrm{dp}\[r\] &=
/// \\min\_{0\\le l\\lt r} (\\tau(r)-\\tau(l)) - (l-1)\\cdot(\\sigma(r)-\\sigma(l)) \\\\
/// &= \\min\_{0\\le l\\lt r} ((l-1)\\cdot\\sigma(l)-\\tau(l) - (l-1)\\cdot\\sigma(r))+\\tau(r)
/// \\end{aligned} $$
/// とできる。よって、上記の枠組みで $p(j) = (j-1)\\cdot\\sigma(j)-\\tau(j)$、$q(j)=-(j-1)$、
/// $r(i)=\\sigma(i)$、$s(i)=\\tau(i)$ としたものと見なせ、$\\sigma(\\bullet)$ や $\\tau(\\bullet)$
/// の計算を適切に高速化すれば、$O(n\\log(n))$ 時間で解ける。
///
/// # Examples
/// ```
/// use nekolib::ds::IncrementalLineSet;
///
/// let mut ls = IncrementalLineSet::new();
/// assert_eq!(ls.min(0), None);
///
/// ls.push((2, 2));
/// assert_eq!(ls.min(0), Some(2));
/// assert_eq!(ls.min(2), Some(6));
///
/// ls.push((1, 3));
/// assert_eq!(ls.min(0), Some(2));
/// assert_eq!(ls.min(2), Some(5));
/// assert_eq!(ls.min(5), Some(8));
///
/// ls.push((-1, 10));
/// assert_eq!(ls.min(2), Some(5));
/// assert_eq!(ls.min(5), Some(5));
///
/// assert_eq!(
///     format!("{:?}", ls),
///     r"{\x. 2x+2: ..1, \x. x+3: ..3, \x. -x+10: ..2147483647}"
/// );
/// ```
///
/// # References
/// - <https://noshi91.hatenablog.com/entry/2021/03/23/200810>
#[derive(Clone, Default)]
pub struct IncrementalLineSet<I> {
    f: BTreeMap<I, I>,
    range: BTreeBimap<I, I>,
}

impl<I: ChtInt> IncrementalLineSet<I> {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, (a, b): (I, I)) {
        if self.f.is_empty() {
            let max = I::oo();
            self.f.insert(a, b);
            self.range.insert(a, max);
            return;
        }
        if self.unused((a, b)) {
            return;
        }
        self.remove_unused((a, b));
        self.insert((a, b));
    }
    pub fn min(&self, x: I) -> Option<I> {
        let a = *self.range.range_right(x..).next()?.1;
        let b = self.f[&a];
        Some(x.on_line((a, b)))
    }
    pub fn inner_len(&self) -> usize { self.f.len() }

    fn unused(&self, (a, b): (I, I)) -> bool {
        let (&al, &bl) = match self.f.range(a..).next() {
            Some((&al, &bl)) if a == al => return bl <= b,
            Some(s) => s,
            None => return false,
        };
        let (&ar, &br) = match self.f.range(..a).next_back() {
            Some(s) => s,
            None => return false,
        };
        al.right(bl, (a, b)) >= a.right(b, (ar, br))
    }
    fn remove_unused(&mut self, (a, b): (I, I)) {
        self.f.remove(&a);
        self.range.remove_left(&a);

        let mut rm = vec![];
        for ((&all, &bll), (&al, &bl)) in
            self.f.range(a..).skip(1).zip(self.f.range(a..))
        {
            if all.right(bll, (al, bl)) >= al.right(bl, (a, b)) {
                rm.push(al);
            } else {
                break;
            }
        }
        for ((&arr, &brr), (&ar, &br)) in
            self.f.range(..a).rev().skip(1).zip(self.f.range(..a).rev())
        {
            if a.right(b, (ar, br)) >= ar.right(br, (arr, brr)) {
                rm.push(ar);
            } else {
                break;
            }
        }
        for ar in &rm {
            self.f.remove(ar);
            self.range.remove_left(ar);
        }
    }
    fn insert(&mut self, (a, b): (I, I)) {
        if let Some((&al, &bl)) = self.f.range(a..).next() {
            self.range.insert(al, al.right(bl, (a, b)));
        };
        if let Some((&ar, &br)) = self.f.range(..a).next_back() {
            self.range.insert(a, a.right(b, (ar, br)));
        } else {
            self.range.insert(a, I::oo());
        }

        self.f.insert(a, b);
    }
}

struct LineDebugHelper<I>(I, I);

impl<I: ChtInt> Debug for LineDebugHelper<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match (self.0.simplify(), self.1.simplify()) {
            (0, _) => format!("\\x. {:?}", self.1),
            (1, _) => format!("\\x. x{:+?}", self.1),
            (-1, _) => format!("\\x. -x{:+?}", self.1),
            (_, 0) => format!("\\x. {:?}x", self.0),
            _ => format!("\\x. {:?}x{:+?}", self.0, self.1),
        };
        f.write_str(&s)
    }
}

impl<I: ChtInt> Debug for IncrementalLineSet<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(
                self.f
                    .iter()
                    .rev()
                    .zip(self.range.range_left(..).rev())
                    .map(|((&a, &b), (&_, &r))| (LineDebugHelper(a, b), ..r)),
            )
            .finish()
    }
}

pub trait ChtInt: Copy + Ord + Default + Debug {
    fn oo() -> Self;
    fn right(self, b: Self, line1: (Self, Self)) -> Self;
    fn on_line(self, line: (Self, Self)) -> Self;
    fn simplify(self) -> i8;
}

macro_rules! impl_cht_int {
    ( $($tt:tt)* ) => { $(
        impl ChtInt for $tt {
            // std::$tt::MAX が 1.43.0 で AtCoder は 1.42.0 なのがつらい。
            fn oo() -> $tt {
                let w = (0 as $tt).count_zeros();
                ((1 as $tt) << (w - 1)).wrapping_sub(1)
            }
            fn right(self, b: Self, (ar, br): (Self, Self)) -> Self {
                // a > ar
                let a = self;
                (br - b).div_euclid(a - ar)
            }
            fn on_line(self, (a, b): (Self, Self)) -> Self { a * self + b }
            fn simplify(self) -> i8 {
                match self {
                    0 => 0,
                    1 => 1,
                    -1 => -1,
                    _ => 2,
                }
            }
        }
    )* };
}

impl_cht_int! { i8 i16 i32 i64 i128 isize }

#[test]
fn test_simple() {
    let mut ls = IncrementalLineSet::new();
    assert_eq!(ls.min(1), None);

    let mut f = std::iter::successors(Some(185_i32), |&x| {
        Some((x * 291 + 748) % 93739)
    })
    .map(|x| x % 300 - 150);

    let mut naive = vec![];
    for _ in 0..5000 {
        let a = f.next().unwrap();
        let b = f.next().unwrap();
        ls.push((a, b));
        naive.push((a, b));
        for x in -100..=100 {
            let expected = naive.iter().map(|&(a, b)| a * x + b).min();
            let got = ls.min(x);
            assert_eq!(got, expected);
        }
    }
}

#[test]
fn test_cross() {
    // 一点でたくさんの直線が交差する場合のテストを書く
    let mut ls = IncrementalLineSet::new();
    // (0, 0) でたくさん交わるようにする
    ls.push((0, 0));
    for a in 1..1000 {
        ls.push((a, 0));
        assert_eq!(ls.inner_len(), 2);
    }
    for a in 1..1000 {
        ls.push((-a, 0));
        assert_eq!(ls.inner_len(), 2);
    }
}

#[test]
fn test_many() {
    // 傾きが 1 ずつ異なる直線がたくさん使われる場合のテストを書く
    let mut ls = IncrementalLineSet::new();
    // (0, 0), (1, -1), (2, -3), (3, -6), (4, -10), ...
    let mut y = 0;
    let x_max = 1000;
    for x in 0..=x_max {
        let a = -x;
        y += a;
        // (x, y) を通り、傾きが a
        // Y - y = a (X - x)
        // Y = a X - a x + y
        ls.push((a, -a * x + y));
        // (-x-1, y) を通り、傾きが -a
        ls.push((-a, -a * x + y - a));
        assert_eq!(ls.inner_len(), (2 * x + 1) as usize);
    }
    for x in -x_max..=x_max {
        let y = -x * (x + 1) / 2;
        assert_eq!(ls.min(x), Some(y));
    }
}

#[test]
fn test_frac() {
    // ある直線が最小となる区間が格子点を含まない場合のテストを書く
    let mut ls = IncrementalLineSet::new();
    ls.push((2, 1)); // [..., -1, 1, 3, ...]
    ls.push((-5, 6)); // [..., 11, 6, 1, ...]
    ls.push((0, 3)); // [..., 3, 3, 3, ...]
    assert_eq!(ls.inner_len(), 2);
}

#[test]
fn test_below() {
    let mut ls = IncrementalLineSet::new();
    ls.push((0, 2));
    assert_eq!(ls.min(10), Some(2));
    ls.push((0, 4));
    assert_eq!(ls.min(10), Some(2));
    ls.push((0, 1));
    assert_eq!(ls.min(10), Some(1));
    assert_eq!(ls.inner_len(), 1);
}
