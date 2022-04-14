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
/// # Idea
/// - `RangeFrom<u32>` などに関しては [指数探索](https://rsk0315.hatenablog.com/entry/2021/12/19/124017)。
/// - `Range<f64>` などに関しては [ビット表現を整数と見て二分探索](https://rsk0315.hatenablog.com/entry/2022/04/07/004618)。
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
/// // 1.584962500721156
/// assert!(((1.0_f64..2.0).bisect(pred) - lg3).abs() <= 1e-16);
/// assert_eq!((1.0_f64..2.0).bisect(pred), lg3); // !
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

macro_rules! impl_bisect_float {
    (
        $(
            (
                $fty:ty, $ity:ty, $uty:ty, $w:literal,
                $f2u:ident, $u2f:ident, $mask:ident
            ),
        )*
    ) => { $(
        impl Bisect for Range<$fty> {
            type Input = $fty;
            type Output = $fty;
            fn bisect(&self, pred: impl Fn(&$fty) -> bool) -> $fty {
                let Range { start, end } = *self;
                let start = $f2u(start);
                let end = $f2u(end);
                $u2f((start..end).bisect(|&u| pred(&$u2f(u))))
            }
        }
        fn $mask(u: $uty) -> $uty {
            ((u as $ity >> ($w - 1)) as $uty >> 1) | 1 << ($w - 1)
        }
        fn $f2u(f: $fty) -> $uty {
            let u = f.to_bits();
            u ^ $mask(u)
        }
        fn $u2f(u: $uty) -> $fty { <$fty>::from_bits(u ^ $mask(!u)) }
    )* };
}

impl_bisect_float! {
    (f32, i32, u32, 32, f2u32, u2f32, mask32),
    (f64, i64, u64, 64, f2u64, u2f64, mask64),
}

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
        // println!("{:.40}", 3.0_f64.log2());
        // println!("{:.40}", (1.0_f64..2.0).bisect(pred));
    }
    {
        assert_eq!([0, 1, 4, 5, 9].bisect(|&x: &i32| x < 5), 3);
        assert_eq!((0..100).bisect(|&x: &i32| x * x < 200), 15);
        assert_eq!((0..).bisect(|&x: &i32| x * x < 200), 15);
    }
    {
        let pred = |&x: &i32| x * x < 200;
        assert_eq!((0..100).bisect(pred), 15);
        assert_eq!((0..).bisect(pred), 15);
    }
}
