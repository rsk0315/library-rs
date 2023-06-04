//! LARSCH algorithm。

use std::cell::RefCell;
use std::ops::Add;
use std::rc::{Rc, Weak};

enum Reduce<T, F, W> {
    Reduce0E,
    Reduce1E(Reduce1<T, F, W>),
    Reduce2E(Reduce2<T, F, W>),
}
use Reduce::{Reduce0E, Reduce1E, Reduce2E};

struct Fetcher<T, F, W> {
    dp: RefCell<Vec<Option<T>>>,
    map: F,
    trans: W,
}

struct Reduce1<T, F, W> {
    n: usize,
    i: usize,
    j0: usize,
    j1: usize,
    reduce: Box<Reduce<T, F, W>>,
    row_shift: u32,
    col_map: RefCell<Vec<usize>>,
    c: RefCell<Vec<usize>>,
    fetcher: Weak<RefCell<Fetcher<T, F, W>>>,
}

struct Reduce2<T, F, W> {
    n: usize,
    i: usize,
    j: usize,
    srow: Vec<usize>,
    scol: Vec<usize>,
    reduce: Box<Reduce<T, F, W>>,
    row_shift: u32,
    col_map: RefCell<Vec<usize>>,
    c: RefCell<Vec<usize>>,
    fetcher: Weak<RefCell<Fetcher<T, F, W>>>,
}

/// LARSCH algorithm。
///
/// $E\[i] = F\[j] + W(j, i)$ ($0\\le j<i<n$) と書ける DP を計算する。
/// ここで、$F\[j]$ は $E\[j]$ から高速に計算できる値とし、$W$ は concave QI
/// を満たすとする。
///
/// `todo!()` ちゃんと書く
///
/// $\\gdef\\DP{\operatorname{dp}}$
/// 行列 $M\[i, j]$ は、$\\DP\[j]$ から $\\DP\[i]$ に遷移するときのコストを表し、
/// 値がオンラインに定まる。この行列は concave totally monotone になっている。
///
/// この行列の row minima を求めるのに相当する。
///
/// `(argmin, min): (Vec<usize>, Vec<T>)` を返すので、`i = n - 1` から始めて
/// `argmin[i] -> i` の遷移を辿ることで復元も可能。
///
/// # Notes
///
/// SMAWK のオンライン版。SMAWK もそのうち書く。
///
/// # Examples
/// ```
/// use nekolib::algo::Larsch;
///
/// let a = vec![1_i64, 2, 3, 4, 5];
/// let c = 6;
/// let n = a.len();
///
/// let map = |&x: &i64| x;
/// let trans = |il: usize, ir: usize| (a[il] - a[ir]).pow(2) + c;
/// let (argmin, min) = Larsch::new(n - 1, n - 1, 0, map, trans).solve();
///
/// assert_eq!(argmin, &[0, 0, 0, 0, 2]);
/// assert_eq!(min, &[0, 7, 10, 15, 20]);
/// ```
///
/// 愚直に書いたときの DP がどんな感じかを書いておくと役立つ場合がありそう。`todo!()`
///
/// # Complexity
/// $E\[\\bullet]$ の計算を $O(1)$ time として $O(n)$ time。
///
/// # References
/// - Larmore, Lawrence L., and Baruch Schieber. "On-line dynamic programming with applications to the prediction of RNA secondary structure." *Journal of Algorithms* 12, no. 3 (1991): 490--515.
///
/// ## See also
/// - <https://noshi91.github.io/Library/algorithm/larsch.cpp>
///
/// # Applications
/// [Examples](#examples) にある EDPC-Z の例の他、[AOJ 3086](https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=3086)
/// などにも利用できる。
///
/// ```
/// use std::cmp::Reverse;
///
/// use nekolib::{algo::Larsch, ds::N1Rmq};
///
/// let a = vec![1, 1, 5, 5, 1, 1];
/// let l = 2;
/// let n = a.len();
///
/// let a: Vec<_> = a.into_iter().map(|ai| Reverse(ai)).collect();
/// let rmq: N1Rmq<_> = a.into();
/// let oo = 10_i64.pow(18);
///
/// let map = |&x: &i64| x;
/// let trans = |il: usize, ir: usize| {
///     if ir < il + l { oo } else { -rmq.min(il, ir).0 }
/// };
/// let (argmin, min) = Larsch::new(n, n, 0, map, trans).solve();
/// let max: Vec<_> = min.into_iter().map(|x| -x).collect();
///
/// assert_eq!(argmin, &[0, 0, 0, 0, 2, 3, 3]);
/// assert_eq!(max, &[0, -oo, 1, 5, 6, 10, 10]);
/// ```
pub struct Larsch<T, F, W> {
    n: usize,
    reduce: Box<Reduce<T, F, W>>,
    fetcher: Rc<RefCell<Fetcher<T, F, W>>>,
}

impl<T, F, W> Reduce<T, F, W>
where
    F: Fn(&T) -> T,
    W: Fn(usize, usize) -> T,
    T: Add<Output = T> + Ord,
{
    fn new(
        n: usize,
        m: usize,
        row_shift: u32,
        col_map: &[usize],
        fetcher: Weak<RefCell<Fetcher<T, F, W>>>,
    ) -> Self {
        if n == 0 && m == 1 {
            Reduce0E
        } else if n >= m {
            Reduce1E(Reduce1::new(n, m, row_shift, col_map, fetcher))
        } else {
            Reduce2E(Reduce2::new(n, m, row_shift, col_map, fetcher))
        }
    }

    fn next(&mut self) -> Option<usize> {
        match self {
            Reduce0E => None,
            Reduce1E(r) => r.next(),
            Reduce2E(r) => r.next(),
        }
    }

    fn set_col_map(&mut self, i_par: usize, i_child: usize) {
        match self {
            Reduce0E => {}
            Reduce1E(r) => r.set_col_map(i_par, i_child),
            Reduce2E(r) => r.set_col_map(i_par, i_child),
        }
    }

    fn set_c(&mut self, i: usize, j: usize) {
        match self {
            Reduce0E => {}
            Reduce1E(r) => r.set_c(i, j),
            Reduce2E(r) => r.set_c(i, j),
        }
    }
}

impl<T, F, W> Fetcher<T, F, W>
where
    F: Fn(&T) -> T,
    W: Fn(usize, usize) -> T,
    T: Add<Output = T> + Ord,
{
    fn new(dp: Vec<Option<T>>, map: F, trans: W) -> Self {
        let dp = RefCell::new(dp);
        Self { dp, map, trans }
    }

    fn fetch(&self, i: usize, j: usize) -> T {
        let f = (self.map)(self.dp.borrow()[j].as_ref().unwrap());
        let w = (self.trans)(j, i);
        let res = f + w;
        res
    }

    fn set(&self, i: usize, val: T) { self.dp.borrow_mut()[i] = Some(val); }

    fn finish(self) -> Vec<T> {
        self.dp.into_inner().into_iter().map(Option::unwrap).collect()
    }
}

impl<T, F, W> Larsch<T, F, W>
where
    F: Fn(&T) -> T,
    W: Fn(usize, usize) -> T,
    T: Add<Output = T> + Ord,
{
    pub fn new(n: usize, m: usize, init: T, map: F, trans: W) -> Self {
        let dp: Vec<_> = (0..=n).map(|_| None).collect();
        let fetcher = Fetcher::new(dp, map, trans);
        fetcher.set(0, init);
        let fetcher = Rc::new(RefCell::new(fetcher));
        let row_shift = 0;
        let col_map: Vec<_> = (0..m).collect();
        let reduce =
            Reduce::new(n, m, row_shift, &col_map, Rc::downgrade(&fetcher));
        Self { n, reduce: Box::new(reduce), fetcher }
    }

    pub fn solve(mut self) -> (Vec<usize>, Vec<T>) {
        let mut argmin = vec![0; self.n + 1];
        for i in 1..=self.n {
            self.reduce.set_c(i, i - 1);
            argmin[i] = self.reduce.next().unwrap();
            let x = self.fetcher.borrow().fetch(i, argmin[i]);
            self.fetcher.borrow().set(i, x);
        }

        let dp: Vec<_> =
            Rc::try_unwrap(self.fetcher).ok().unwrap().into_inner().finish();

        (argmin, dp)
    }
}

impl<T, F, W> Reduce1<T, F, W>
where
    F: Fn(&T) -> T,
    W: Fn(usize, usize) -> T,
    T: Add<Output = T> + Ord,
{
    fn new(
        n: usize,
        m: usize,
        row_shift: u32,
        col_map: &[usize],
        fetcher: Weak<RefCell<Fetcher<T, F, W>>>,
    ) -> Self {
        let reduce = Reduce::new(
            n / 2,
            m,
            row_shift + 1,
            col_map,
            Weak::clone(&fetcher),
        );

        Self {
            n,
            i: 1,
            j0: 0,
            j1: 0,
            reduce: Box::new(reduce),
            row_shift,
            col_map: RefCell::new(col_map.to_vec()),
            c: RefCell::new(vec![0; n + 1]),
            fetcher,
        }
    }

    fn c(&self, i: usize) -> usize { self.c.borrow()[i] }

    fn set_col_map(&mut self, i_par: usize, i_child: usize) {
        self.col_map.borrow_mut()[i_child] = i_par;
        self.reduce.set_col_map(i_par, i_child);
    }

    fn set_c(&mut self, i: usize, j: usize) {
        self.c.borrow_mut()[i] = j;
        if i / 2 + 1 <= self.n / 2 {
            self.reduce.set_c(i / 2 + 1, j);
        }
    }

    fn fetch(&self, i: usize, j: usize) -> T {
        let i = i << self.row_shift;
        let j = self.col_map.borrow()[j];
        self.fetcher.upgrade().unwrap().borrow().fetch(i, j)
    }

    fn next(&mut self) -> Option<usize> {
        if self.i > self.n {
            return None;
        }

        let j_range = if self.i % 2 == 1 {
            if self.i % 2 == 1 && self.i / 2 + 1 <= self.n / 2 {
                self.reduce.set_c(self.i / 2 + 1, self.c(self.i));
            }
            self.j1 = self.reduce.next().unwrap_or_else(|| self.c(self.i));
            (self.j0..=self.j1).chain(None)
        } else {
            (self.c(self.i - 1) + 1..=self.c(self.i)).chain(Some(self.j1))
        };

        let j = j_range.min_by_key(|&j| (self.fetch(self.i, j), j)).unwrap();
        if self.i % 2 == 0 {
            self.j0 = j;
        }

        self.i += 1;
        Some(j)
    }
}

impl<T, F, W> Reduce2<T, F, W>
where
    F: Fn(&T) -> T,
    W: Fn(usize, usize) -> T,
    T: Add<Output = T> + Ord,
{
    fn new(
        n: usize,
        _m: usize,
        row_shift: u32,
        col_map: &[usize],
        fetcher: Weak<RefCell<Fetcher<T, F, W>>>,
    ) -> Self {
        let reduce =
            Reduce::new(n, n, row_shift, &vec![0; n], Weak::clone(&fetcher));

        Self {
            n,
            i: 1,
            j: 0,
            srow: vec![0],
            scol: vec![0],
            reduce: Box::new(reduce),
            row_shift,
            col_map: RefCell::new(col_map.to_vec()),
            c: RefCell::new(vec![0; n + 1]),
            fetcher,
        }
    }

    fn c(&self, i: usize) -> usize { self.c.borrow()[i] }

    fn is_finite(&self, i: usize, j: usize) -> bool {
        j <= self.c(i.min(self.i))
    }

    fn set_col_map(&mut self, i_par: usize, i_child: usize) {
        self.col_map.borrow_mut()[i_child] = i_par;
    }

    fn set_c(&mut self, i: usize, j: usize) { self.c.borrow_mut()[i] = j; }

    fn fetch(&self, i: usize, j: usize) -> Option<T> {
        if !self.is_finite(i, j) {
            return None;
        }

        let i = i << self.row_shift;
        let j = self.col_map.borrow()[j];

        Some(self.fetcher.upgrade().unwrap().borrow().fetch(i, j))
    }

    fn is_lt(
        &self,
        (il, jl): (usize, usize),
        (ir, jr): (usize, usize),
    ) -> bool {
        if (il, jl) == (ir, jr) {
            return false;
        }
        match (self.fetch(il, jl), self.fetch(ir, jr)) {
            (Some(fl), Some(fr)) => (fl, jl) < (fr, jr),
            (_, None) => true,  // finite < oo
            (None, _) => false, // oo < finite
        }
    }

    fn next(&mut self) -> Option<usize> {
        if self.i > self.n {
            return None;
        }

        // c[0] := -1
        let jl = if self.i == 1 { 0 } else { self.c(self.i - 1) + 1 };
        let jr = self.c(self.i);
        for j in jl..=jr {
            loop {
                let r = *self.srow.last().unwrap();
                let c = *self.scol.last().unwrap();
                if !self.is_lt((r, j), (r, c)) {
                    break;
                }
                self.srow.pop();
                self.scol.pop();
            }
            if *self.srow.last().unwrap() < self.n {
                let r = *self.srow.last().unwrap();
                let i_ = (r + 1..).find(|&i_| self.is_finite(i_, j)).unwrap();
                self.srow.push(i_);
                self.scol.push(j);
            }
        }
        if self.j + 1 < self.srow.len() && self.srow[self.j + 1] == self.i {
            self.j += 1;
            let tmp = self.col_map.borrow()[self.scol[self.j]];
            self.reduce.set_col_map(tmp, self.j - 1);
        }

        self.reduce.set_c(self.i, self.j - 1);
        let j = self.reduce.next().unwrap();
        self.i += 1;
        Some(self.scol[j + 1])
    }
}
