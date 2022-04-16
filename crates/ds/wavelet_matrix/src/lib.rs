//! wavelet matrix。

use std::ops::{Index, Range, RangeBounds, RangeInclusive};

use buf_range::bounds_within;
use count::{Count, Count3way, Count3wayResult};
use find_nth::FindNth;
use quantile::Quantile;
use rs_dict::RsDict;

/// wavelet matrix。
///
/// 整数に関する多くの区間クエリを処理できる。
///
/// # Examples
/// ```
/// use nekolib::ds::WaveletMatrix;
/// use nekolib::traits::{Count3way, FindNth, Quantile};
///
/// let wm: WaveletMatrix<u32> = vec![1, 8, 4, 9, 2, 7, 5, 2].into();
///
/// assert_eq!(wm.count_3way(2.., 5).lt(), 3); // [4, _, 2, _, _, _, 2]
///
/// let c3 = wm.count_3way(..6, 2..=7); // [1, 8, 4, 9, 2, 7]
/// assert_eq!(c3.lt(), 1); // [1, _, _, _, _, _]
/// assert_eq!(c3.eq(), 3); // [_, _, 4, _, 2, 7]
/// assert_eq!(c3.gt(), 2); // [_, 8, _, 9, _, _]
/// assert_eq!(c3.le(), 4);
/// assert_eq!(c3.ge(), 5);
///
/// assert_eq!(wm.quantile(2..=4, 0), Some(2)); // [_, _, 2]
/// assert_eq!(wm.quantile(2..=4, 1), Some(4)); // [4, _, _]
/// assert_eq!(wm.quantile(2..=4, 2), Some(9)); // [_, 9, _]
/// assert_eq!(wm.quantile(2..=4, 3), None);
///
/// assert_eq!(wm.find_nth(3.., 2, 0), Some(4));
/// assert_eq!(wm.find_nth(3.., 2, 1), Some(7));
/// assert_eq!(wm.find_nth(4.., 2, 0), Some(4));
/// assert_eq!(wm.find_nth(5.., 2, 0), Some(7));
/// assert_eq!(wm.find_nth(5.., 2, 1), None);
/// ```
pub struct WaveletMatrix<I> {
    len: usize,
    bitlen: usize,
    buf: Vec<RsDict>,
    zeros: Vec<usize>,
    orig: Vec<I>,
}

impl<I: WmInt> From<Vec<I>> for WaveletMatrix<I> {
    fn from(orig: Vec<I>) -> Self {
        let len = orig.len();
        let bitlen =
            orig.iter().map(|ai| ai.bitlen()).max().unwrap_or(0) as usize;
        let mut whole = orig.clone();
        let mut zeros = vec![0; bitlen];
        let mut buf = vec![];
        for i in (0..bitlen).rev() {
            let mut zero = vec![];
            let mut one = vec![];
            let mut vb = vec![false; len];
            for (j, aj) in whole.into_iter().enumerate() {
                (if aj.test(i) { &mut one } else { &mut zero }).push(aj);
                vb[j] = aj.test(i);
            }
            zeros[i] = zero.len();
            buf.push(vb.into());
            whole = zero;
            whole.append(&mut one);
        }
        buf.reverse();
        Self { len, bitlen, buf, zeros, orig }
    }
}

impl<I: WmInt> Count<I> for WaveletMatrix<I> {
    fn count(&self, range: impl RangeBounds<usize>, value: I) -> usize {
        self.count_3way(range, value).eq()
    }
}

impl<I: WmInt> Count<RangeInclusive<I>> for WaveletMatrix<I> {
    fn count(
        &self,
        range: impl RangeBounds<usize>,
        value: RangeInclusive<I>,
    ) -> usize {
        self.count_3way(range, value).eq()
    }
}

impl<I: WmInt> Count3way<I> for WaveletMatrix<I> {
    fn count_3way(
        &self,
        range: impl RangeBounds<usize>,
        value: I,
    ) -> Count3wayResult {
        let Range { start, end } = bounds_within(range, self.len);
        let (lt, gt) = self.count_3way_internal(start..end, value);
        let eq = (end - start) - (lt + gt);
        Count3wayResult::new(lt, eq, gt)
    }
}

impl<I: WmInt> Count3way<RangeInclusive<I>> for WaveletMatrix<I> {
    fn count_3way(
        &self,
        range: impl RangeBounds<usize>,
        value: RangeInclusive<I>,
    ) -> Count3wayResult {
        let Range { start: il, end: ir } = bounds_within(range, self.len);
        let vl = *value.start();
        let vr = *value.end();
        let lt = self.count_3way_internal(il..ir, vl).0;
        let gt = self.count_3way_internal(il..ir, vr).1;
        let eq = (ir - il) - (lt + gt);
        Count3wayResult::new(lt, eq, gt)
    }
}

impl<I: WmInt> WaveletMatrix<I> {
    fn count_3way_internal(
        &self,
        Range { mut start, mut end }: Range<usize>,
        value: I,
    ) -> (usize, usize) {
        if start == end {
            return (0, 0);
        }
        if value.bitlen() > self.bitlen {
            return (end - start, 0);
        }
        let mut lt = 0;
        let mut gt = 0;
        for i in (0..self.bitlen).rev() {
            let tmp = end - start;
            if !value.test(i) {
                start = self.buf[i].rank(start, 0);
                end = self.buf[i].rank(end, 0);
            } else {
                start = self.zeros[i] + self.buf[i].rank(start, 1);
                end = self.zeros[i] + self.buf[i].rank(end, 1);
            }
            *(if value.test(i) { &mut lt } else { &mut gt }) +=
                tmp - (end - start);
        }
        (lt, gt)
    }
}

impl<I: WmInt> Quantile for WaveletMatrix<I> {
    type Output = I;
    fn quantile(
        &self,
        range: impl RangeBounds<usize>,
        mut n: usize,
    ) -> Option<I> {
        let Range { mut start, mut end } = bounds_within(range, self.len);
        if end - start <= n {
            return None;
        }
        let mut res = I::zero();
        for i in (0..self.bitlen).rev() {
            let z = self.buf[i].count(start..end, 0);
            if n < z {
                start = self.buf[i].rank(start, 0);
                end = self.buf[i].rank(end, 0);
            } else {
                res.set(i);
                start = self.zeros[i] + self.buf[i].rank(start, 1);
                end = self.zeros[i] + self.buf[i].rank(end, 1);
                n -= z;
            }
        }
        Some(res)
    }
}

impl<I: WmInt> WaveletMatrix<I> {
    pub fn xored_quantile(
        &self,
        range: impl RangeBounds<usize>,
        mut n: usize,
        x: I,
    ) -> Option<I> {
        let Range { mut start, mut end } = bounds_within(range, self.len);
        if end - start <= n {
            return None;
        }
        let mut res = I::zero();
        for i in (0..self.bitlen).rev() {
            let z = self.buf[i].count(start..end, 0);
            if !x.test(i) {
                if n < z {
                    start = self.buf[i].rank(start, 0);
                    end = self.buf[i].rank(end, 0);
                } else {
                    res.set(i);
                    start = self.zeros[i] + self.buf[i].rank(start, 1);
                    end = self.zeros[i] + self.buf[i].rank(end, 1);
                    n -= z;
                }
            } else {
                let z = (end - start) - z;
                if n < z {
                    start = self.zeros[i] + self.buf[i].rank(start, 1);
                    end = self.zeros[i] + self.buf[i].rank(end, 1);
                } else {
                    res.set(i);
                    start = self.buf[i].rank(start, 0);
                    end = self.buf[i].rank(end, 0);
                    n -= z;
                }
            }
        }
        Some(res)
    }
}

impl<I: WmInt> FindNth<I> for WaveletMatrix<I> {
    fn find_nth(
        &self,
        range: impl RangeBounds<usize>,
        value: I,
        n: usize,
    ) -> Option<usize> {
        let start = bounds_within(range, self.len).start;
        let (lt, gt) = self.count_3way_internal(0..start, value);
        let offset = start - (lt + gt);
        Some(self.select(value, n + offset + 1)? - 1)
    }
}

impl<I: WmInt> WaveletMatrix<I> {
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }

    pub fn rank(&self, end: usize, value: I) -> usize {
        self.count(0..end, value)
    }
    pub fn select(&self, value: I, mut n: usize) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }
        let (lt, gt) = self.count_3way_internal(0..self.len, value);
        let count = self.len - (lt + gt);
        if count < n {
            return None;
        }
        let si = self.start_pos(value);
        let value0 = value.test(0) as u64;
        n += self.buf[0].rank(si, value0);
        n = self.buf[0].select(value0, n).unwrap();

        for i in 1..self.bitlen {
            if !value.test(i) {
                n = self.buf[i].select(0, n).unwrap();
            } else {
                n -= self.zeros[i];
                n = self.buf[i].select(1, n).unwrap();
            }
        }
        Some(n)
    }
    fn start_pos(&self, value: I) -> usize {
        let mut start = 0;
        let mut end = 0;
        for i in (1..self.bitlen).rev() {
            if !value.test(i) {
                start = self.buf[i].rank(start, 0);
                end = self.buf[i].rank(end, 0);
            } else {
                start = self.zeros[i] + self.buf[i].rank(start, 1);
                end = self.zeros[i] + self.buf[i].rank(end, 1);
            }
        }
        start
    }
}

impl<I: WmInt> Index<usize> for WaveletMatrix<I> {
    type Output = I;
    fn index(&self, i: usize) -> &I { &self.orig[i] }
}

pub trait WmInt: Copy {
    fn test(self, i: usize) -> bool;
    fn set(&mut self, i: usize);
    fn bitlen(self) -> usize;
    fn zero() -> Self;
}

macro_rules! impl_wm_int {
    ( $( $ty:ty )* ) => { $(
        impl WmInt for $ty {
            fn test(self, i: usize) -> bool { self >> i & 1 != 0 }
            fn set(&mut self, i: usize) { *self |= 1 << i; }
            fn bitlen(self) -> usize {
                let w = (0 as $ty).count_zeros() as usize;
                if self.test(w - 1) {
                    w
                } else {
                    (self + 1).next_power_of_two().trailing_zeros() as usize
                }
            }
            fn zero() -> $ty { 0 }
        }
    )* };
}

impl_wm_int! { u8 u16 u32 u64 u128 usize }

#[test]
fn test_simple() {
    let n = 300;
    let f = std::iter::successors(Some(296), |&x| Some((x * 258 + 185) % 397))
        .map(|x| x & 7);
    let buf: Vec<_> = f.take(n).collect();
    let wm: WaveletMatrix<u32> = buf.clone().into();
    for start in 0..n {
        let mut count = vec![0; 8];
        for end in start..=n {
            for xl in 0..=7 {
                for xr in xl..=7 {
                    let lt: usize = count[..xl as usize].iter().sum();
                    let gt: usize = count[xr as usize + 1..].iter().sum();
                    let eq = (end - start) - (lt + gt);
                    let c3 = Count3wayResult::new(lt, eq, gt);
                    assert_eq!(wm.count_3way(start..end, xl..=xr), c3);
                }

                let lt: usize = count[..xl as usize].iter().sum();
                let gt: usize = count[xl as usize + 1..].iter().sum();
                let eq = (end - start) - (lt + gt);
                let c3 = Count3wayResult::new(lt, eq, gt);
                assert_eq!(wm.count(start..end, xl), eq);
                assert_eq!(wm.count(start..end, xl..=xl), eq);
                assert_eq!(wm.count_3way(start..end, xl), c3);
                assert_eq!(wm.count_3way(start..end, xl..=xl), c3);
            }

            if end < n {
                count[buf[end] as usize] += 1;
            }
        }
    }

    for start in 0..n {
        let mut count = vec![0; 8];
        for end in start..n {
            let x = buf[end];
            assert_eq!(wm.find_nth(start.., x, count[x as usize]), Some(end));
            count[x as usize] += 1;
        }
        for x in 0..8 {
            assert_eq!(wm.find_nth(start.., x, count[x as usize]), None);
        }
    }

    for start in 0..n {
        for end in start..n {
            let mut tmp = buf[start..end].to_vec();
            tmp.sort_unstable();
            for i in 0..tmp.len() {
                assert_eq!(wm.quantile(start..end, i), Some(tmp[i]));
            }
            assert_eq!(wm.quantile(start..end, tmp.len()), None);
        }
    }

    for start in 0..n {
        for end in start..n {
            for x in 0..8 {
                let mut tmp: Vec<_> =
                    buf[start..end].iter().map(|&y| x ^ y).collect();
                tmp.sort_unstable();
                for i in 0..tmp.len() {
                    assert_eq!(
                        wm.xored_quantile(start..end, i, x),
                        Some(tmp[i])
                    );
                }
                assert_eq!(wm.xored_quantile(start..end, tmp.len(), x), None);
            }
        }
    }
}

#[test]
fn test_count() {
    let n = 8;
    let c3 = |lt, eq, gt| Count3wayResult::new(lt, eq, gt);

    let zero: WaveletMatrix<u8> = vec![0; n].into();
    assert_eq!(zero.count_3way(.., 0), c3(0, n, 0));
    assert_eq!(zero.count_3way(.., 0..=0), c3(0, n, 0));
    assert_eq!(zero.count_3way(.., 1), c3(n, 0, 0));
    assert_eq!(zero.count_3way(.., 1..=1), c3(n, 0, 0));
    assert_eq!(zero.count_3way(.., 254), c3(n, 0, 0));
    assert_eq!(zero.count_3way(.., 254..=254), c3(n, 0, 0));
    assert_eq!(zero.count_3way(.., 255), c3(n, 0, 0));
    assert_eq!(zero.count_3way(.., 255..=255), c3(n, 0, 0));

    let full: WaveletMatrix<u8> = vec![!0; n].into();
    assert_eq!(full.count_3way(.., 0), c3(0, 0, n));
    assert_eq!(full.count_3way(.., 0..=0), c3(0, 0, n));
    assert_eq!(full.count_3way(.., 1), c3(0, 0, n));
    assert_eq!(full.count_3way(.., 1..=1), c3(0, 0, n));
    assert_eq!(full.count_3way(.., 254), c3(0, 0, n));
    assert_eq!(full.count_3way(.., 254..=254), c3(0, 0, n));
    assert_eq!(full.count_3way(.., 255), c3(0, n, 0));
    assert_eq!(full.count_3way(.., 255..=255), c3(0, n, 0));
}
