//! 二分探索。

use std::ops::{Range, RangeFrom, RangeTo};

/// 二分探索。
///
/// $\\gdef\\halfopen#1#2{[#1, #2)}$
/// 述語 $f$ は、ある $x$ に対して次が成り立つとする。
/// - $y\\in\\halfopen{L}{x} \\implies f(y)$
/// - $y\\in\\halfopen{x}{R} \\implies \\lnot f(y)$
///
/// この $x$ を返す。
///
/// # Implementation notes
/// `impl Bisect for Range<f64>`（精度を指定しない方）に関して、回数指定が下手そう。
///
/// # Examples
/// ```
/// use nekolib::traits::Bisect;
///
/// let pred = |&x: &i32| x * x < 200;
/// assert_eq!((0..100).bisect(pred), 15);
/// assert_eq!((0..).bisect(pred), 15);
///
/// let a = [0, 1, 4, 5, 5, 5, 9];
/// let pred = |&x: &i32| x < 5;
/// assert_eq!(a.bisect(pred), 3);
/// assert_eq!(a[5..].bisect(pred), 0); // [5, 9]
/// assert_eq!(a[..0].bisect(pred), 0); // []
///
/// let pred = |&x: &f64| 2.0_f64.powf(x) < 3.0;
/// let lg3 = 3.0_f64.log2();
/// assert!(((1.0_f64..2.0).bisect(pred) - lg3).abs() <= 5e-324);
/// assert!(((1.0_f64..2.0, 1e-9).bisect(pred) - lg3).abs() <= 1e-9);
/// ```
pub trait Bisect {
    type Input;
    type Output;
    fn bisect(&self, pred: impl Fn(&Self::Input) -> bool) -> Self::Output;
}

macro_rules! impl_bisect_int {
    ($t:ty) => {
        impl Bisect for Range<$t> {
            type Input = $t;
            type Output = $t;
            fn bisect(&self, pred: impl Fn(&$t) -> bool) -> $t {
                let Range { start: mut ok, end: mut bad } = *self;
                if !pred(&ok) {
                    return ok;
                }
                while bad - ok > 1 {
                    let mid = ok + (bad - ok) / 2;
                    *(if pred(&mid) { &mut ok } else { &mut bad }) = mid;
                }
                bad
            }
        }

        impl Bisect for RangeFrom<$t> {
            type Input = $t;
            type Output = $t;
            fn bisect(&self, pred: impl Fn(&$t) -> bool) -> $t {
                let RangeFrom { start: ok } = *self;
                if !pred(&ok) {
                    return ok;
                }
                let mut w = 1;
                while pred(&(ok + w)) {
                    w *= 2;
                }
                (ok..ok + w).bisect(pred)
            }
        }

        impl Bisect for RangeTo<$t> {
            type Input = $t;
            type Output = $t;
            fn bisect(&self, pred: impl Fn(&$t) -> bool) -> $t {
                let RangeTo { end: bad } = *self;
                if pred(&bad) {
                    return bad;
                }
                let mut w = 1;
                while !pred(&(bad - w)) {
                    w *= 2;
                }
                (bad - w..bad).bisect(pred)
            }
        }

    };
    ( $( $t:ty )* ) => { $( impl_bisect_int! { $t } )* };
}

impl_bisect_int! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

trait BisectTimes {
    type Input;
    type Output;
    fn bisect_times(
        &self,
        times: u32,
        pred: impl Fn(&Self::Input) -> bool,
    ) -> Self::Output;
}

macro_rules! impl_bisect_float {
    ($t:ty) => {
        impl Bisect for Range<$t> {
            type Input = $t;
            type Output = $t;
            fn bisect(&self, pred: impl Fn(&$t) -> bool) -> $t {
                let Range { start, end } = *self;
                let times = (end - start).log2().ceil() as u32 + 64 + 1;
                self.bisect_times(times, pred)
            }
        }

        impl Bisect for (Range<$t>, $t) {
            type Input = $t;
            type Output = $t;
            fn bisect(&self, pred: impl Fn(&$t) -> bool) -> $t {
                let (Range { start, end }, eps) = *self;
                let times = ((end - start) / eps).log2().ceil() as u32 + 1;
                (start..end).bisect_times(times, pred)
            }
        }

        impl BisectTimes for Range<$t> {
            type Input = $t;
            type Output = $t;
            fn bisect_times(&self, times: u32, pred: impl Fn(&$t) -> bool) -> $t {
                let Range { start: mut ok, end: mut bad } = *self;
                for _ in 0..times {
                    let mid = 0.5 * (ok + bad);
                    *(if pred(&mid) { &mut ok } else { &mut bad }) = mid;
                }
                bad
            }
        }
    };
    ( $( $t:ty )* ) => { $( impl_bisect_float! { $t } )* };
}

impl_bisect_float! { f32 f64 }

impl<T> Bisect for [T] {
    type Input = T;
    type Output = usize;
    fn bisect(&self, pred: impl Fn(&T) -> bool) -> usize {
        if self.is_empty() || !pred(&self[0]) {
            return 0;
        }
        let mut ok = 0;
        let mut bad = self.len();
        while bad - ok > 1 {
            let mid = ok + (bad - ok) / 2;
            *(if pred(&self[mid]) { &mut ok } else { &mut bad }) = mid;
        }
        bad
    }
}

#[test]
fn test() {
    {
        let pred = |&x: &i64| x < 100;
        assert_eq!((0_i64..200).bisect(pred), 100);
        assert_eq!((0_i64..).bisect(pred), 100);
        assert_eq!((..200_i64).bisect(pred), 100);
    }

    {
        let pred = |&x: &i64| x.abs() < 100;
        assert_eq!((0_i64..200).bisect(pred), 100);
        assert_eq!((0_i64..).bisect(pred), 100);
        assert_eq!((..0_i64).bisect(|x| !pred(x)), -99);
    }

    {
        let pred = |&x: &i64| x < 5;
        let a = vec![0, 1, 4, 5, 5, 9];
        assert_eq!(a.bisect(pred), 3);
        assert_eq!(a[4..].bisect(pred), 0);
    }

    {
        let pred = |&x: &f64| 2.0_f64.powf(x) < 3.0;
        assert!(((1.0_f64..2.0).bisect(pred) - 3.0_f64.log2()) <= 5.0e-324);
        println!("{}", (1.0_f64..2.0).bisect(pred));
    }
    {
        assert_eq!([0, 1, 4, 5, 9].bisect(|&x: &i32| x < 5), 3);
        assert_eq!((0..100).bisect(|&x: &i32| x * x < 200), 15);
        assert_eq!((0..).bisect(|&x: &i32| x * x < 200), 15);
    }
    {
        let lg3 = |&x: &f64| 2.0_f64.powf(x) < 3.0;
        let range = 1.0_f64..2.0;
        assert!((range.bisect(lg3) - 3.0_f64.log2()).abs() <= 5e-324);
        assert!(((range, 1e-9).bisect(lg3) - 3.0_f64.log2()).abs() <= 1e-9);
    }
    {
        let pred = |&x: &i32| x * x < 200;
        assert_eq!((0..100).bisect(pred), 15);
        assert_eq!((0..).bisect(pred), 15);
    }
}
