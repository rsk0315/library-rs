//! wavelet matrix。

use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Range, RangeBounds,
};

use buf_range::bounds_within;
use count::{Count, Count3way, Count3wayResult};
use find_nth::FindNth;
use nth_min::NthMin;
use rs_dict::RsDict;

/// wavelet matrix。
pub struct WaveletMatrix {
    len: usize,
    bitlen: usize,
    buf: Vec<RsDict>,
    zeros: Vec<usize>,
}

impl From<Vec<u128>> for WaveletMatrix {
    fn from(orig: Vec<u128>) -> Self {
        if orig.is_empty() {
            return Self { len: 0, bitlen: 0, buf: vec![], zeros: vec![] };
        }

        let len = orig.len();
        let mut whole = orig.clone();
        let &max = orig.iter().max().unwrap();
        let bitlen = if max >= 1 << 127 {
            128
        } else {
            (max + 1).next_power_of_two().trailing_zeros() as usize
        };
        let mut zeros = vec![0; bitlen];
        let mut buf = vec![];
        for i in (0..bitlen).rev() {
            let mut zero = vec![];
            let mut one = vec![];
            let mut vb = vec![];
            for j in 0..len {
                (if whole[j] >> i & 1 == 1 { &mut one } else { &mut zero })
                    .push(whole[j]);
                vb[j] = whole[j] >> i & 1 == 1;
            }
            zeros[i] = zero.len();
            buf.push(vb.into());
            whole = zero;
            whole.append(&mut one);
        }
        Self { len, bitlen, buf, zeros }
    }
}

impl<R: RangeBounds<u128>> Count<R> for WaveletMatrix {
    fn count(&self, range: impl RangeBounds<usize>, value: R) -> usize {
        self.count_3way(range, value).eq()
    }
}

impl<R: RangeBounds<u128>> Count3way<R> for WaveletMatrix {
    fn count_3way(
        &self,
        range: impl RangeBounds<usize>,
        value: R,
    ) -> Count3wayResult {
        let Range { start, end } = bounds_within(range, self.len);
        let len = end - start;
        let lt = match value.start_bound() {
            Included(&x) => self.count_3way_internal(start..end, x).0,
            Excluded(&std::u128::MAX) => len,
            Excluded(&x) => self.count_3way_internal(start..end, x + 1).0,
            Unbounded => 0,
        };
        let gt = match value.end_bound() {
            Included(&x) => self.count_3way_internal(start..end, x).1,
            Excluded(&0) => len,
            Excluded(&x) => self.count_3way_internal(start..end, x - 1).1,
            Unbounded => 0,
        };
        let eq = len - (lt + gt);
        Count3wayResult::new(lt, eq, gt)
    }
}

impl WaveletMatrix {
    fn count_3way_internal(
        &self,
        Range { mut start, mut end }: Range<usize>,
        value: u128,
    ) -> (usize, usize) {
        if start == end {
            return (0, 0);
        }
        let mut lt = 0;
        let mut gt = 0;
        for i in (0..self.bitlen).rev() {
            let tmp = end - start;
            if value >> i & 1 == 0 {
                start = self.buf[i].rank(start, 0);
                end = self.buf[i].rank(end, 0);
            } else {
                start = self.zeros[i] + self.buf[i].rank(start, 1);
                end = self.zeros[i] + self.buf[i].rank(end, 1);
            }
            *(if value >> i & 1 == 0 { &mut lt } else { &mut gt }) +=
                tmp - (end - start);
        }
        (lt, gt)
    }
}

impl NthMin for WaveletMatrix {
    type Output = u128;
    fn nth_min(
        &self,
        range: impl RangeBounds<usize>,
        mut n: usize,
    ) -> Option<u128> {
        let Range { mut start, mut end } = bounds_within(range, self.len);
        if end - start <= n {
            return None;
        }
        let mut res = 0;
        for i in (0..self.bitlen).rev() {
            let z = self.buf[i].count(start..end, 0);
            if n < z {
                start = self.buf[i].rank(start, 0);
                end = self.buf[i].rank(end, 0);
            } else {
                res |= 1_u128 << i;
                start = self.zeros[i] + self.buf[i].rank(start, 1);
                end = self.zeros[i] + self.buf[i].rank(end, 1);
                n -= z;
            }
        }
        Some(res)
    }
}

impl FindNth<u128> for WaveletMatrix {
    fn find_nth(
        &self,
        range: impl RangeBounds<usize>,
        value: u128,
        n: usize,
    ) -> Option<usize> {
        let Range { start, end } = bounds_within(range, self.len);
        let (lt, gt) = self.count_3way_internal(0..start, value);
        let offset = start - (lt + gt);
        Some(self.select(end, value, n + offset + 1)? - 1)
    }
}

impl WaveletMatrix {
    pub fn select(
        &self,
        end: usize,
        value: u128,
        mut n: usize,
    ) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }
        let (lt, gt) = self.count_3way_internal(0..end, value);
        let count = end - (lt + gt);
        if count < n {
            return None;
        }
        let si = self.start_pos(value);
        let value0 = (value & 1) as u64;
        n += self.buf[0].rank(si, value0);
        n = self.buf[0].select(value0, n);

        for i in 1..self.bitlen {
            if value >> i & 1 == 0 {
                n = self.buf[i].select(0, n);
            } else {
                n -= self.zeros[i];
                n = self.buf[i].select(1, n);
            }
        }
        Some(n)
    }
    fn start_pos(&self, value: u128) -> usize {
        let mut start = 0;
        let mut end = 0;
        for i in (1..self.bitlen).rev() {
            if value >> i & 1 == 0 {
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
