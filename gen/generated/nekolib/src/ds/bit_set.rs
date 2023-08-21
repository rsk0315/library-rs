//! bit set。

use super::super::utils::buf_range;

use std::cmp::Ordering;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Range,
    RangeBounds, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use buf_range::{bounds_within, check_bounds};

type Word = u64;
const WORD_SIZE: usize = (0 as Word).count_zeros() as usize;

/// Bit set。
///
/// # Implementation notes
/// `&` `|` `^` について、左辺の capacity を持つ新たな `BitSet` を返すため、可換でない。
/// 可換にすることにすると、 `x = x | y` と `x |= y` の整合性を取りたくなり、`|=`
/// の操作でも capacity を変化させるのが妥当になるが、あまりうれしくなさそう？
///
/// capacity は暗黙に変わらないような設計にしているが、`eq()` などでは capacity
/// は無視するべき？ 立っているビットが同じかを比較するときに `eq()`
/// で済むのがうれしいか、`cmp().is_eq()` にするか？ そこを短くするために capacity
/// の比較を別でやる必要がある方がつらいか？
///
/// [`u128` での実装](https://atcoder.jp/contests/past202203-open/submissions/33482505) より
/// [`u64` での実装](https://atcoder.jp/contests/past202203-open/submissions/33482482)
/// の方が高速だったので、とりあえずそうしている。`BitSet<u128>` のようにすると煩雑になりそう。
#[derive(Default, Clone, Eq)]
pub struct BitSet {
    capacity: usize,
    len: usize,
    buf: Vec<Word>,
    autofix: bool,
}

impl BitSet {
    pub fn new(capacity: usize) -> Self {
        let buf = vec![0; (capacity + WORD_SIZE - 1) / WORD_SIZE];
        Self { capacity, len: 0, buf, autofix: true }
    }

    pub fn insert(&mut self, index: usize) {
        check_bounds(index, self.capacity);

        let (wi, bi) = (index / WORD_SIZE, index % WORD_SIZE);
        if self.buf[wi] >> bi & 1 == 0 {
            self.buf[wi] |= 1 << bi;
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) {
        check_bounds(index, self.capacity);

        let (wi, bi) = (index / WORD_SIZE, index % WORD_SIZE);
        if self.buf[wi] >> bi & 1 != 0 {
            self.buf[wi] &= !(1 << bi);
            self.len -= 1;
        }
    }

    #[must_use]
    pub fn contains(&self, index: usize) -> bool {
        check_bounds(index, self.capacity);

        let (wi, bi) = (index / WORD_SIZE, index % WORD_SIZE);
        self.buf[wi] >> bi & 1 != 0
    }

    #[must_use]
    pub fn len(&self) -> usize { self.len }
    #[must_use]
    pub fn is_empty(&self) -> bool { self.len == 0 }
    #[must_use]
    pub fn capacity(&self) -> usize { self.capacity }

    // simple bit operations (assignment)
    pub fn and_assign(&mut self, other: &Self) {
        for (lhs, &rhs) in self.buf.iter_mut().zip(&other.buf) {
            *lhs &= rhs;
        }
        if self.buf.len() > other.buf.len() {
            for e in &mut self.buf[other.buf.len()..] {
                *e = 0;
            }
        }
        self.fixup();
    }
    pub fn or_assign(&mut self, other: &Self) {
        for (lhs, &rhs) in self.buf.iter_mut().zip(&other.buf) {
            *lhs |= rhs;
        }
        self.fixup();
    }
    pub fn ior_assign(&mut self, other: &Self) { self.or_assign(other) }
    pub fn xor_assign(&mut self, other: &Self) {
        for (lhs, &rhs) in self.buf.iter_mut().zip(&other.buf) {
            *lhs ^= rhs;
        }
        self.fixup();
    }
    pub fn sub_assign(&mut self, other: &Self) {
        for (lhs, &rhs) in self.buf.iter_mut().zip(&other.buf) {
            *lhs &= !rhs;
        }
        self.fixup();
    }
    pub fn not_assign(&mut self) {
        for lhs in &mut self.buf {
            *lhs = !*lhs;
        }
        self.fixup();
    }
    pub fn shl_assign(&mut self, shl: usize) {
        check_bounds(shl, self.capacity);
        let (quot, rem) = (shl / WORD_SIZE, shl % WORD_SIZE);
        let buf = &mut self.buf;
        for i in (quot..buf.len()).rev() {
            let mut tmp = buf[i - quot] << rem;
            if rem > 0 && i - quot > 0 {
                tmp |= buf[i - quot - 1] >> (WORD_SIZE - rem);
            }
            buf[i] = tmp;
        }
        for e in &mut buf[..quot] {
            *e = 0;
        }
        self.fixup();
    }
    pub fn shr_assign(&mut self, shr: usize) {
        check_bounds(shr, self.capacity);
        let (quot, rem) = (shr / WORD_SIZE, shr % WORD_SIZE);
        let buf = &mut self.buf;
        let mid = buf.len() - quot;
        for i in 0..mid {
            let mut tmp = buf[i + quot] >> rem;
            if rem > 0 && i + quot + 1 < buf.len() {
                tmp |= buf[i + quot + 1] << (WORD_SIZE - rem);
            }
            buf[i] = tmp;
        }
        for e in &mut buf[mid..] {
            *e = 0;
        }
        self.fixup();
    }

    // simple bit operations (non-assignment)
    #[must_use]
    pub fn and(&self, other: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.and_assign(other);
        tmp
    }
    #[must_use]
    pub fn or(&self, other: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.or_assign(other);
        tmp
    }
    #[must_use]
    pub fn ior(&self, other: &Self) -> Self { self.or(other) }
    #[must_use]
    pub fn xor(&self, other: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.xor_assign(other);
        tmp
    }
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        let mut tmp = self.clone();
        tmp.sub_assign(other);
        tmp
    }
    #[must_use]
    pub fn not(&self) -> Self {
        let mut tmp = self.clone();
        tmp.not_assign();
        tmp
    }
    #[must_use]
    pub fn shl(&self, shl: usize) -> Self {
        let mut tmp = self.clone();
        tmp.shl_assign(shl);
        tmp
    }
    #[must_use]
    pub fn shr(&self, shr: usize) -> Self {
        let mut tmp = self.clone();
        tmp.shr_assign(shr);
        tmp
    }

    pub fn reserve_exact(&mut self, new_capacity: usize) {
        let new_buf_len = (new_capacity + WORD_SIZE - 1) / WORD_SIZE;
        if self.buf.len() > new_buf_len {
            for x in &self.buf[new_buf_len..] {
                self.len -= x.count_ones() as usize;
            }
        }
        self.buf.resize(new_buf_len, 0);
        self.capacity = new_capacity;
        self.fixup_last();
    }

    pub fn reserve(&mut self, at_least: usize) {
        if self.capacity < at_least {
            self.reserve_exact(at_least);
        }
    }

    pub fn autofix(&mut self, enable: bool) {
        let fix_now = !self.autofix && enable;
        self.autofix = enable;
        if fix_now {
            self.fixup();
        }
    }

    fn fixup(&mut self) {
        if !self.autofix {
            return;
        }
        self.fixup_count();
        self.fixup_last();
    }

    fn fixup_last(&mut self) {
        let rem = self.capacity % WORD_SIZE;
        if rem == 0 {
            return;
        }
        // `rem != 0` implies `self.buf.len() > 0`
        let last = self.buf.last_mut().unwrap();
        self.len -= last.count_ones() as usize;
        *last &= !(!0 << rem);
        self.len += last.count_ones() as usize;
    }

    fn fixup_count(&mut self) {
        self.len = self.buf.iter().map(|x| x.count_ones() as usize).sum();
    }

    #[must_use]
    pub fn words(&self, range: impl RangeBounds<usize>) -> Words<'_> {
        let range = bounds_within(range, self.capacity);
        Words::new(self, range)
    }

    #[must_use]
    pub fn indices(&self, range: impl RangeBounds<usize>) -> Indices<'_> {
        let range = bounds_within(range, self.capacity);
        Indices::new(self, range)
    }

    fn single_word(&self, start: usize, end: usize) -> Word {
        // [start..end] の bit からなる word を返す。範囲外は 0 でうめる。
        // 0 <= end - start <= WORD_SIZE と end <= self.capacity は仮定する。
        if start == end {
            return 0;
        }

        let (ws, bs) = (start / WORD_SIZE, start % WORD_SIZE);
        let (we, be) = (end / WORD_SIZE, end % WORD_SIZE);
        let len = end - start;
        let w = self.buf[ws];
        if be == 0 {
            if bs == 0 { w } else { w >> (WORD_SIZE - len) }
        } else if ws == we {
            (w >> bs) & !(!0 << len)
        } else {
            // e.g.: (LSB) _____xxx xx______ (MSB); bs: 5, be: 2
            (w >> bs) | (self.buf[we] & !(!0 << be)) << (WORD_SIZE - bs)
        }
    }

    fn single_word_bsf(&self, start: usize, end: usize) -> Option<usize> {
        let w = self.single_word(start, end);
        if w == 0 { None } else { Some(start + bsf(w)) }
    }

    fn single_word_bsr(&self, start: usize, end: usize) -> Option<usize> {
        let w = self.single_word(start, end);
        if w == 0 { None } else { Some(start + bsr(w)) }
    }

    #[must_use]
    pub fn find_first(&self, range: impl RangeBounds<usize>) -> Option<usize> {
        let Range { start, end } = bounds_within(range, self.capacity);
        if start >= end {
            return None;
        }

        let s_ceil = (start + WORD_SIZE - 1) / WORD_SIZE;
        let e_floor = end / WORD_SIZE;
        if s_ceil > e_floor {
            return self.single_word_bsf(start, end);
        }

        let first = self.single_word_bsf(start, s_ceil * WORD_SIZE);
        let middle = self.buf[s_ceil..e_floor]
            .iter()
            .zip(s_ceil..e_floor)
            .filter(|&(&w, _)| w != 0)
            .map(|(&w, i)| i * WORD_SIZE + bsf(w));
        let last = self.single_word_bsf(e_floor * WORD_SIZE, end);
        first.into_iter().chain(middle).chain(last).next()
    }

    #[must_use]
    pub fn find_last(&self, range: impl RangeBounds<usize>) -> Option<usize> {
        let Range { start, end } = bounds_within(range, self.capacity);
        if start >= end {
            return None;
        }

        let s_ceil = (start + WORD_SIZE - 1) / WORD_SIZE;
        let e_floor = end / WORD_SIZE;
        if s_ceil > e_floor {
            return self.single_word_bsr(start, end);
        }

        let first = self.single_word_bsr(start, s_ceil * WORD_SIZE);
        let middle = self.buf[s_ceil..e_floor]
            .iter()
            .zip(s_ceil..e_floor)
            .filter(|&(&w, _)| w != 0)
            .map(|(&w, i)| i * WORD_SIZE + bsr(w))
            .rev();
        let last = self.single_word_bsr(e_floor * WORD_SIZE, end);
        last.into_iter().chain(middle).chain(first).next()
    }
}

pub struct Words<'a> {
    range: (usize, usize),
    bit_set: &'a BitSet,
}

impl<'a> Words<'a> {
    fn new(bit_set: &'a BitSet, Range { start, end }: Range<usize>) -> Self {
        Self { range: (start, end), bit_set }
    }
}

impl Iterator for Words<'_> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, end) = self.range;
        if start >= end {
            return None;
        }
        self.range.0 = end.min(start + WORD_SIZE);
        Some(self.bit_set.single_word(start, self.range.0))
    }
}

impl DoubleEndedIterator for Words<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (start, end) = self.range;
        if start >= end {
            return None;
        }
        self.range.1 = start.max(end.saturating_sub(WORD_SIZE));
        Some(self.bit_set.single_word(self.range.1, end))
    }
}

pub struct Indices<'a> {
    range: (usize, usize),
    bit_set: &'a BitSet,
}

impl<'a> Indices<'a> {
    pub fn new(
        bit_set: &'a BitSet,
        Range { start, end }: Range<usize>,
    ) -> Self {
        Self { bit_set, range: (start, end) }
    }
}

impl Iterator for Indices<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, end) = self.range;
        if start >= end {
            return None;
        }
        let res = self.bit_set.find_first(start..end);
        self.range.0 = res.map(|i| i + 1).unwrap_or(self.range.1);
        res
    }
}

impl DoubleEndedIterator for Indices<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (start, end) = self.range;
        if start >= end {
            return None;
        }
        let res = self.bit_set.find_last(start..end);
        self.range.1 = res.unwrap_or(self.range.0);
        res
    }
}

impl Ord for BitSet {
    fn cmp(&self, other: &Self) -> Ordering {
        // iter() の辞書順で比較することにしたい。
        // words().map(|x| x.reverse_bits()) の辞書順で比較すると、
        // 0 の word がたくさんあって buf.len() の差があるときに相違が出る。
        //
        // [..self.buf.len().min(other.buf.len())] までで比較して、
        // 同じなら、len の辞書順で比較すればよさそう。
        let min_len = self.buf.len().min(other.buf.len());
        (self.buf[..min_len].iter().map(|x| x.reverse_bits()))
            .cmp(other.buf[..min_len].iter().map(|x| x.reverse_bits()))
            .then_with(|| self.len.cmp(&other.len))
    }
}

impl PartialOrd for BitSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool { self.cmp(&other) == Ordering::Equal }
}

macro_rules! fused_shl_bitop {
    ($self:ident, $lhs:expr, $rhs:expr, $shl:expr, $bitop:expr) => {
        let (quot, rem) = ($shl / WORD_SIZE, $shl % WORD_SIZE);
        let rhs_upper = $rhs.len() + quot + rem.min(1);
        for i in (quot..$lhs.len().min(rhs_upper)).rev() {
            let mut tmp = 0;
            if i - quot < $rhs.len() {
                tmp |= $rhs[i - quot] << rem;
            }
            if rem > 0 && i - quot >= 1 {
                tmp |= $rhs[i - quot - 1] >> (WORD_SIZE - rem);
            }
            $lhs[i] = $bitop($lhs[i], tmp);
        }
        if $bitop(0b01, 0) != 0b01 {
            if quot >= $lhs.len().min(rhs_upper) {
                for lhs in &mut $lhs {
                    *lhs = $bitop(*lhs, 0);
                }
            } else {
                if quot < $lhs.len() {
                    for lhs in &mut $lhs[..quot] {
                        *lhs = $bitop(*lhs, 0);
                    }
                }
                if $lhs.len() > rhs_upper {
                    for lhs in &mut $lhs[rhs_upper..] {
                        *lhs = $bitop(*lhs, 0);
                    }
                }
            }
        }
        $self.fixup();
    };
}
macro_rules! fused_shr_bitop {
    ($self:ident, $lhs:expr, $rhs:expr, $shr:expr, $bitop:expr) => {
        let (quot, rem) = ($shr / WORD_SIZE, $shr % WORD_SIZE);
        let mid = $lhs.len().min($rhs.len() - quot);
        for i in 0..mid {
            let mut tmp = $rhs[i + quot] >> rem;
            if rem > 0 && i + quot + 1 < $rhs.len() {
                tmp |= $rhs[i + quot + 1] << (WORD_SIZE - rem);
            }
            $lhs[i] = $bitop($lhs[i], tmp);
        }
        if $bitop(0b01, 0) != 0b01 {
            for lhs in &mut $lhs[mid..] {
                *lhs = $bitop(*lhs, 0);
            }
        }
        $self.fixup();
    };
}

macro_rules! impl_fused {
    ( ($name:ident, self $bin:tt self << $sh:ident) ) => {
        pub fn $name(&mut self, $sh: usize) {
            check_bounds($sh, self.capacity);
            fused_shl_bitop! {
                self, self.buf, self.buf, $sh, |x, y| x $bin y
            }
        }
    };
    ( ($name:ident, self $bin:tt other << $sh:ident) ) => {
        pub fn $name(&mut self, $sh: usize, other: &Self) {
            check_bounds($sh, other.capacity);
            fused_shl_bitop! {
                self, self.buf, other.buf, $sh, |x, y| x $bin y
            }
        }
    };
    ( ($name:ident, self $bin:tt self >> $sh:ident) ) => {
        pub fn $name(&mut self, $sh: usize) {
            check_bounds($sh, self.capacity);
            fused_shr_bitop! {
                self, self.buf, self.buf, $sh, |x, y| x $bin y
            }
        }
    };
    ( ($name:ident, self $bin:tt other >> $sh:ident) ) => {
        pub fn $name(&mut self, $sh: usize, other: &Self) {
            check_bounds($sh, other.capacity);
            fused_shr_bitop! {
                self, self.buf, other.buf, $sh, |x, y| x $bin y
            }
        }
    };
    ( ($op:ident, $op_assign:ident, $op_self:ident, $op_self_assign:ident) ) => {
        pub fn $op(&self, sh: usize, other: &Self) -> Self {
            let mut tmp = self.clone();
            tmp.$op_assign(sh, other);
            tmp
        }
        pub fn $op_self(&self, sh: usize) -> Self {
            let mut tmp = self.clone();
            tmp.$op_self_assign(sh);
            tmp
        }
    };
    ( $( ( $( $tt:tt )* ), )* ) => { $( impl_fused!{ ( $( $tt )* ) } )* }
}

// fused bit operations
impl BitSet {
    impl_fused! {
        (shl_and_self_assign, self & self << x),
        (shl_ior_self_assign, self | self << x),
        (shl_xor_self_assign, self ^ self << x),
        (shr_and_self_assign, self & self >> x),
        (shr_ior_self_assign, self | self >> x),
        (shr_xor_self_assign, self ^ self >> x),

        (shl_and_assign, self & other << x),
        (shl_ior_assign, self | other << x),
        (shl_xor_assign, self ^ other << x),
        (shr_and_assign, self & other >> x),
        (shr_ior_assign, self | other >> x),
        (shr_xor_assign, self ^ other >> x),

        (shl_or_self_assign, self | self << x),
        (shr_or_self_assign, self | self >> x),
        (shl_or_assign,  self | other << x),
        (shr_or_assign,  self | other >> x),

        (shl_and, shl_and_assign, shl_and_self, shl_and_self_assign),
        (shl_ior, shl_ior_assign, shl_ior_self, shl_ior_self_assign),
        (shl_xor, shl_xor_assign, shl_xor_self, shl_xor_self_assign),
        (shl_sub, shl_sub_assign, shl_sub_self, shl_sub_self_assign),
        (shr_and, shr_and_assign, shr_and_self, shr_and_self_assign),
        (shr_ior, shr_ior_assign, shr_ior_self, shr_ior_self_assign),
        (shr_xor, shr_xor_assign, shr_xor_self, shr_xor_self_assign),
        (shr_sub, shr_sub_assign, shr_sub_self, shr_sub_self_assign),

        (shl_or, shl_or_assign, shl_or_self, shl_or_self_assign),
        (shr_or, shr_or_assign, shr_or_self, shr_or_self_assign),
    }

    pub fn shl_sub_assign(&mut self, shl: usize, other: &Self) {
        self.shl_op_assign(shl, other, |x, y| x & !y)
    }
    pub fn shl_sub_self_assign(&mut self, shl: usize) {
        self.shl_op_self_assign(shl, |x, y| x & !y)
    }
    pub fn shr_sub_assign(&mut self, shr: usize, other: &Self) {
        self.shr_op_assign(shr, other, |x, y| x & !y)
    }
    pub fn shr_sub_self_assign(&mut self, shr: usize) {
        self.shr_op_self_assign(shr, |x, y| x & !y)
    }

    pub fn shl_op_assign(
        &mut self,
        shl: usize,
        other: &Self,
        f: impl Fn(Word, Word) -> Word,
    ) {
        fused_shl_bitop!(self, self.buf, other.buf, shl, f);
    }
    pub fn shl_op_self_assign(
        &mut self,
        shl: usize,
        f: impl Fn(Word, Word) -> Word,
    ) {
        fused_shl_bitop!(self, self.buf, self.buf, shl, f);
    }
    pub fn shr_op_assign(
        &mut self,
        shr: usize,
        other: &Self,
        f: impl Fn(Word, Word) -> Word,
    ) {
        fused_shr_bitop!(self, self.buf, other.buf, shr, f);
    }
    pub fn shr_op_self_assign(
        &mut self,
        shr: usize,
        f: impl Fn(Word, Word) -> Word,
    ) {
        fused_shr_bitop!(self, self.buf, self.buf, shr, f);
    }

    #[must_use]
    pub fn shl_op(
        &self,
        sh: usize,
        other: &Self,
        f: impl Fn(Word, Word) -> Word,
    ) -> Self {
        let mut tmp = self.clone();
        tmp.shl_op_assign(sh, other, f);
        tmp
    }
    #[must_use]
    pub fn shl_op_self(
        &self,
        sh: usize,
        f: impl Fn(Word, Word) -> Word,
    ) -> Self {
        let mut tmp = self.clone();
        tmp.shl_op_self_assign(sh, f);
        tmp
    }
    #[must_use]
    pub fn shr_op(
        &self,
        sh: usize,
        other: &Self,
        f: impl Fn(Word, Word) -> Word,
    ) -> Self {
        let mut tmp = self.clone();
        tmp.shr_op_assign(sh, other, f);
        tmp
    }
    #[must_use]
    pub fn shr_op_self(
        &self,
        sh: usize,
        f: impl Fn(Word, Word) -> Word,
    ) -> Self {
        let mut tmp = self.clone();
        tmp.shr_op_self_assign(sh, f);
        tmp
    }
}

impl fmt::Binary for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.capacity;
        for &w in &self.buf[..n / WORD_SIZE] {
            write!(f, "{0:01$b}", w.reverse_bits(), WORD_SIZE)?;
        }
        let rem = n % WORD_SIZE;
        if rem != 0 {
            let w = self.buf[n / WORD_SIZE].reverse_bits() >> (WORD_SIZE - rem);
            write!(f, "{0:01$b}", w, rem)?;
        }
        Ok(())
    }
}

impl fmt::Debug for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.indices(..)).finish()
    }
}

impl Extend<usize> for BitSet {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = usize>,
    {
        for i in iter {
            self.insert(i);
        }
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = usize>,
    {
        let mut res = BitSet::new(0);
        for i in iter {
            res.reserve(i + 1);
            res.insert(i);
        }
        res
    }
}

macro_rules! impl_binary_op {
    ( $( ($trait:ident, $method:ident, $op_assign:ident, $op:ident), )* ) => { $(
        impl $trait::<&BitSet> for BitSet {
            type Output = BitSet;
            fn $method(mut self, other: &Self) -> Self {
                self.$op_assign(other);
                self
            }
        }
        impl $trait::<BitSet> for BitSet {
            type Output = BitSet;
            fn $method(mut self, other: Self) -> Self {
                self.$op_assign(&other);
                self
            }
        }
        impl<'a> $trait::<&'a BitSet> for &'a BitSet {
            type Output = BitSet;
            fn $method(self, other: Self) -> BitSet { self.$op(other) }
        }
        impl<'a> $trait::<BitSet> for &'a BitSet {
            type Output = BitSet;
            fn $method(self, other: BitSet) -> BitSet { self.$op(&other) }
        }
    )* }
}

macro_rules! impl_binary_op_assign {
    ( $( ($trait:ident, $method:ident, $op_assign:ident), )* ) => { $(
        impl $trait::<&BitSet> for BitSet {
            fn $method(&mut self, other: &Self) {
                self.$op_assign(other);
            }
        }
        impl $trait::<BitSet> for BitSet {
            fn $method(&mut self, other: Self) {
                self.$op_assign(&other);
            }
        }
    )* }
}

macro_rules! impl_shift {
    ( $( ($trait:ident, $method:ident, $op_assign:ident, $op:ident), )* ) => { $(
        impl $trait::<usize> for BitSet {
            type Output = Self;
            fn $method(mut self, sh: usize) -> BitSet {
                self.$op_assign(sh);
                self
            }
        }
        impl $trait::<usize> for &'_ BitSet {
            type Output = BitSet;
            fn $method(self, sh: usize) -> BitSet { self.$op(sh) }
        }
    )* }
}

macro_rules! impl_shift_assign {
    ( $( ($trait:ident, $method:ident, $op_assign:ident), )* ) => { $(
        impl $trait::<usize> for BitSet {
            fn $method(&mut self, sh: usize) {
                self.$op_assign(sh);
            }
        }
    )* }
}

impl_binary_op! {
    (BitAnd, bitand, and_assign, and),
    (BitOr, bitor, or_assign, or),
    (BitXor, bitxor, xor_assign, xor),
    (Sub, sub, sub_assign, sub),
}

impl_binary_op_assign! {
    (BitAndAssign, bitand_assign, and_assign),
    (BitOrAssign, bitor_assign, or_assign),
    (BitXorAssign, bitxor_assign, xor_assign),
    (SubAssign, sub_assign, sub_assign),
}

impl_shift! {
    (Shl, shl, shl_assign, shl),
    (Shr, shr, shr_assign, shr),
}

impl_shift_assign! {
    (ShlAssign, shl_assign, shl_assign),
    (ShrAssign, shr_assign, shr_assign),
}

impl Not for BitSet {
    type Output = Self;
    fn not(mut self) -> Self {
        self.not_assign();
        self
    }
}
impl Not for &'_ BitSet {
    type Output = BitSet;
    fn not(self) -> BitSet { self.clone().not() }
}

fn bsf(w: Word) -> usize { w.trailing_zeros() as usize }
fn bsr(w: Word) -> usize { WORD_SIZE - 1 - w.leading_zeros() as usize }

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::{BitSet, WORD_SIZE};

    #[test]
    fn fmt() {
        let mut bs = BitSet::new(10);
        assert_eq!(format!("{:b}", bs), "0000000000");

        bs.insert(3);
        assert_eq!(format!("{:b}", bs), "0001000000");

        bs.insert(5);
        assert_eq!(format!("{:b}", bs), "0001010000");

        bs.reserve_exact(128);
        assert_eq!(
            format!("{:b}", bs),
            "00010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
        );

        bs.insert(126);
        assert_eq!(
            format!("{:b}", bs),
            "00010100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010"
        );

        bs.reserve_exact(129);
        assert_eq!(
            format!("{:b}", bs),
            "000101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100"
        );

        bs.insert(128);
        assert_eq!(
            format!("{:b}", bs),
            "000101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000101"
        );
    }

    #[test]
    fn basics() {
        let mut bs = BitSet::new(10);
        assert!(bs.is_empty());
        assert!(!bs.contains(3));

        bs.insert(3);
        assert_eq!(bs.len(), 1);
        assert!(bs.contains(3));

        bs.remove(2);
        assert_eq!(bs.len(), 1);

        bs.remove(3);
        assert_eq!(bs.len(), 0);
    }

    const SET: &[usize] = &[
        0, 1, 126, 127, // 0
        128, 129, 253, 255, // 1
        256, 258, 380, 383, // 2
        384, 387, // 3
        513, // 4
        640, // 5
        // 6
        896,  // 7
        1025, // 8
        // 9
        1407, // 10
    ];

    const CONSEC: &[usize] = &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    const FIB: &[usize] = &[0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    const POW2: &[usize] = &[1, 2, 4, 8, 16, 32, 64, 128];

    #[test]
    fn reserve_fix() {
        let set = SET;

        for i in 0..set.len() {
            let mut bs: BitSet = set[..=i].iter().copied().collect();
            assert_eq!(bs.len(), i + 1);

            for j in 0..set.len() {
                let mut bs = bs.clone();
                bs.reserve_exact(set[j] + 1);
                assert_eq!(bs.len(), i.min(j) + 1);
            }
            bs.reserve_exact(0);
            assert_eq!(bs.len(), 0);
        }
    }

    #[test]
    fn indices() {
        let set = SET;

        let n = set.len();
        let mut bs = BitSet::new(set[n - 1] + 1);
        bs.extend(set.iter().copied());

        let fwd: Vec<_> = bs.indices(..).collect();
        let bck: Vec<_> = bs.indices(..).rev().collect();
        let rev_bck: Vec<_> = bck.into_iter().rev().collect();
        assert_eq!(fwd, rev_bck);
    }

    #[test]
    fn words() {
        let set = SET;

        let n = set.len();
        let capacity = set[n - 1] + 1;
        let mut bs = BitSet::new(capacity);
        bs.extend(set.iter().copied());

        // fwd
        for i in 0..WORD_SIZE {
            let actual: Vec<_> = bs.words(i..).collect();
            let expected: Vec<_> = {
                let mut bs = BitSet::new(capacity);
                bs.extend(set.iter().filter_map(|&x| x.checked_sub(i)));
                bs.words(..).collect()
            };
            assert_eq!(actual, expected);
        }

        // bck
        for i in 0..WORD_SIZE {
            let actual: Vec<_> = bs.words(..capacity - i).rev().collect();
            let expected: Vec<_> = {
                let mut bs = BitSet::new(capacity);
                bs.extend(set.iter().map(|&x| x + i).filter(|&x| x < capacity));
                bs.words(i..).rev().collect()
            };
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn cmp() {
        let mut bs1 = BitSet::new(10);
        let mut bs2 = BitSet::new(1000);

        assert!(bs1 == bs2);

        bs2.insert(999);
        assert!(bs1 < bs2);

        bs1.insert(2);
        assert!(bs1 > bs2);

        bs2.insert(1);
        assert!(bs1 < bs2);

        bs2.reserve_exact(1);
        assert!(bs1 > bs2);

        bs1.reserve_exact(0);
        assert!(bs1 == bs2);
    }

    #[test]
    fn eq() {
        let set = SET;
        let n = set.len();
        let m = set[n - 1] + 1;

        let mut bs1 = BitSet::new(m);
        bs1.extend(set.iter().copied());
        let mut bs2 = BitSet::new(m + 1);
        bs2.extend(set.iter().copied());
        assert!(bs1 == bs2);

        bs2.insert(m);
        assert!(bs1 != bs2);

        bs2.reserve_exact(m);
        assert!(bs1 == bs2);
    }

    #[test]
    fn shl() {
        let set = SET;
        let n = set.len();
        let m = set[n - 1] + 1;

        let mut bs1 = BitSet::new(m);
        bs1.extend(set.iter().copied());

        for i in 0..2 * WORD_SIZE {
            let mut bs2 = BitSet::new(m);
            bs2.extend(bs1.indices(..).map(|x| x + i).filter(|&x| x < m));
            assert!(&bs1 << i == bs2);
        }
    }

    #[test]
    fn shr() {
        let set = SET;
        let n = set.len();
        let m = set[n - 1] + 1;

        let mut bs1 = BitSet::new(m);
        bs1.extend(set.iter().copied());

        for i in 0..2 * WORD_SIZE {
            let mut bs2 = BitSet::new(m);
            bs2.extend(bs1.indices(..).filter(|&x| x >= i).map(|x| x - i));
            assert!(&bs1 >> i == bs2);
        }
    }

    #[test]
    fn not() {
        let bs: BitSet = [0, 1, 3, 6].iter().copied().collect();
        assert_eq!(bs.words(..).next(), Some(0b1001011));
        assert_eq!(bs.len(), 4);
        let not_bs = !&bs;
        assert_eq!(not_bs.words(..).next(), Some(0b0110100));
        assert_eq!(not_bs.len(), 3);
    }

    #[test]
    fn binary_ops() {
        let m = 200;
        let (bs1, bs2, bs3) = {
            let mut bs1 = BitSet::new(m);
            bs1.extend(CONSEC.iter().copied());
            let mut bs2 = BitSet::new(m);
            bs2.extend(FIB.iter().copied());
            let mut bs3 = BitSet::new(m);
            bs3.extend(POW2.iter().copied());
            (bs1, bs2, bs3)
        };

        let ts1: BTreeSet<_> = bs1.indices(..).collect();
        let ts2: BTreeSet<_> = bs2.indices(..).collect();
        let ts3: BTreeSet<_> = bs3.indices(..).collect();

        fn is_eq(actual: BitSet, expected: BTreeSet<usize>) -> bool {
            actual.indices(..).eq(expected)
        }

        assert!(is_eq(&bs1 & &bs1, &ts1 & &ts1));
        assert!(is_eq(&bs1 & &bs2, &ts1 & &ts2));
        assert!(is_eq(&bs1 & &bs3, &ts1 & &ts3));
        assert!(is_eq(&bs2 & &bs1, &ts2 & &ts1));
        assert!(is_eq(&bs2 & &bs2, &ts2 & &ts2));
        assert!(is_eq(&bs2 & &bs3, &ts2 & &ts3));
        assert!(is_eq(&bs3 & &bs1, &ts3 & &ts1));
        assert!(is_eq(&bs3 & &bs2, &ts3 & &ts2));
        assert!(is_eq(&bs3 & &bs3, &ts3 & &ts3));

        assert!(is_eq(&bs1 | &bs1, &ts1 | &ts1));
        assert!(is_eq(&bs1 | &bs2, &ts1 | &ts2));
        assert!(is_eq(&bs1 | &bs3, &ts1 | &ts3));
        assert!(is_eq(&bs2 | &bs1, &ts2 | &ts1));
        assert!(is_eq(&bs2 | &bs2, &ts2 | &ts2));
        assert!(is_eq(&bs2 | &bs3, &ts2 | &ts3));
        assert!(is_eq(&bs3 | &bs1, &ts3 | &ts1));
        assert!(is_eq(&bs3 | &bs2, &ts3 | &ts2));
        assert!(is_eq(&bs3 | &bs3, &ts3 | &ts3));

        assert!(is_eq(&bs1 ^ &bs1, &ts1 ^ &ts1));
        assert!(is_eq(&bs1 ^ &bs2, &ts1 ^ &ts2));
        assert!(is_eq(&bs1 ^ &bs3, &ts1 ^ &ts3));
        assert!(is_eq(&bs2 ^ &bs1, &ts2 ^ &ts1));
        assert!(is_eq(&bs2 ^ &bs2, &ts2 ^ &ts2));
        assert!(is_eq(&bs2 ^ &bs3, &ts2 ^ &ts3));
        assert!(is_eq(&bs3 ^ &bs1, &ts3 ^ &ts1));
        assert!(is_eq(&bs3 ^ &bs2, &ts3 ^ &ts2));
        assert!(is_eq(&bs3 ^ &bs3, &ts3 ^ &ts3));

        assert!(is_eq(&bs1 - &bs1, &ts1 - &ts1));
        assert!(is_eq(&bs1 - &bs2, &ts1 - &ts2));
        assert!(is_eq(&bs1 - &bs3, &ts1 - &ts3));
        assert!(is_eq(&bs2 - &bs1, &ts2 - &ts1));
        assert!(is_eq(&bs2 - &bs2, &ts2 - &ts2));
        assert!(is_eq(&bs2 - &bs3, &ts2 - &ts3));
        assert!(is_eq(&bs3 - &bs1, &ts3 - &ts1));
        assert!(is_eq(&bs3 - &bs2, &ts3 - &ts2));
        assert!(is_eq(&bs3 - &bs3, &ts3 - &ts3));
    }

    #[test]
    fn or_capacity() {
        let mut bs1 = BitSet::new(4);
        bs1.extend([0, 3]);
        let mut bs2 = BitSet::new(5);
        bs2.extend([0, 2, 4]);

        assert_eq!((&bs1 | &bs2).words(..).next(), Some(0b1101));
        assert_eq!((&bs2 | &bs1).words(..).next(), Some(0b11101));

        bs1.reserve_exact(5);
        assert_eq!((&bs1 | &bs2).words(..).next(), Some(0b11101));
        assert_eq!((&bs2 | &bs1).words(..).next(), Some(0b11101));
    }

    #[test]
    fn and_capacity() {
        let mut bs1 = BitSet::new(10);
        bs1.extend([1, 2, 4, 8]);
        let mut bs2 = BitSet::new(1000);
        bs2.extend([2, 3, 4, 5, 100, 200, 500, 900]);

        assert!((&bs1 & &bs2).indices(..).eq([2, 4]));
        assert!((&bs2 & &bs1).indices(..).eq([2, 4]));
    }

    #[test]
    fn fused_shift_bitwise() {
        let bs0: BitSet = SET.iter().copied().collect();
        let bs1: BitSet = CONSEC.iter().copied().collect();
        let bs2: BitSet = FIB.iter().copied().collect();
        let bs3: BitSet = POW2.iter().copied().collect();

        fn test_internal(lhs: &BitSet, rhs: &BitSet) -> Result<(), usize> {
            let rhs_x = {
                let mut rhs_x = rhs.clone();
                rhs_x.reserve(lhs.capacity());
                rhs_x
            };

            for i in 0..rhs.capacity() {
                let actual = vec![
                    lhs.shl_and(i, &rhs),
                    lhs.shl_ior(i, &rhs),
                    lhs.shl_xor(i, &rhs),
                    lhs.shl_sub(i, &rhs),
                    lhs.shr_and(i, &rhs),
                    lhs.shr_ior(i, &rhs),
                    lhs.shr_xor(i, &rhs),
                    lhs.shr_sub(i, &rhs),
                    // shl + op
                    lhs.shl_op(i, &rhs, |_, _| 0), //        0000
                    lhs.shl_op(i, &rhs, |x, y| x & y), //    0001
                    lhs.shl_op(i, &rhs, |x, y| x & !y), //   0010
                    lhs.shl_op(i, &rhs, |x, _| x), //        0011
                    lhs.shl_op(i, &rhs, |x, y| !x & y), //   0100
                    lhs.shl_op(i, &rhs, |_, y| y), //        0101
                    lhs.shl_op(i, &rhs, |x, y| x ^ y), //    0110
                    lhs.shl_op(i, &rhs, |x, y| x | y), //    0111
                    lhs.shl_op(i, &rhs, |x, y| !x & !y), //  1000
                    lhs.shl_op(i, &rhs, |x, y| !(x ^ y)), // 1001
                    lhs.shl_op(i, &rhs, |_, y| !y), //       1010
                    lhs.shl_op(i, &rhs, |x, y| x | !y), //   1011
                    lhs.shl_op(i, &rhs, |x, _| !x), //       1100
                    lhs.shl_op(i, &rhs, |x, y| !x | y), //   1101
                    lhs.shl_op(i, &rhs, |x, y| !x | !y), //  1110
                    lhs.shl_op(i, &rhs, |_, _| !0), //       1111
                    // shr + op
                    lhs.shr_op(i, &rhs, |_, _| 0), //        0000
                    lhs.shr_op(i, &rhs, |x, y| x & y), //    0001
                    lhs.shr_op(i, &rhs, |x, y| x & !y), //   0010
                    lhs.shr_op(i, &rhs, |x, _| x), //        0011
                    lhs.shr_op(i, &rhs, |x, y| !x & y), //   0100
                    lhs.shr_op(i, &rhs, |_, y| y), //        0101
                    lhs.shr_op(i, &rhs, |x, y| x ^ y), //    0110
                    lhs.shr_op(i, &rhs, |x, y| x | y), //    0111
                    lhs.shr_op(i, &rhs, |x, y| !x & !y), //  1000
                    lhs.shr_op(i, &rhs, |x, y| !(x ^ y)), // 1001
                    lhs.shr_op(i, &rhs, |_, y| !y), //       1010
                    lhs.shr_op(i, &rhs, |x, y| x | !y), //   1011
                    lhs.shr_op(i, &rhs, |x, _| !x), //       1100
                    lhs.shr_op(i, &rhs, |x, y| !x | y), //   1101
                    lhs.shr_op(i, &rhs, |x, y| !x | !y), //  1110
                    lhs.shr_op(i, &rhs, |_, _| !0), //       1111
                ];
                let mut rhs_l = &rhs_x << i;
                let mut rhs_r = &rhs_x >> i;
                rhs_l.reserve_exact(lhs.capacity());
                rhs_r.reserve_exact(lhs.capacity());
                let rhs_l = &rhs_l;
                let rhs_r = &rhs_r;

                let expected = vec![
                    lhs & rhs_l,
                    lhs | rhs_l,
                    lhs ^ rhs_l,
                    lhs - rhs_l,
                    lhs & rhs_r,
                    lhs | rhs_r,
                    lhs ^ rhs_r,
                    lhs - rhs_r,
                    // shl + op
                    lhs & !lhs,     // 0000
                    lhs & rhs_l,    // 0001
                    lhs & !rhs_l,   // 0010
                    lhs.clone(),    // 0011
                    !lhs & rhs_l,   // 0100
                    rhs_l.clone(),  // 0101
                    lhs ^ rhs_l,    // 0110
                    lhs | rhs_l,    // 0111
                    !lhs & !rhs_l,  // 1000
                    !(lhs ^ rhs_l), // 1001
                    !rhs_l,         // 1010
                    lhs | !rhs_l,   // 1011
                    !lhs,           // 1100
                    !lhs | rhs_l,   // 1101
                    !lhs | !rhs_l,  // 1110
                    lhs | !lhs,     // 1111
                    // shr + op
                    lhs & !lhs,     // 0000
                    lhs & rhs_r,    // 0001
                    lhs & !rhs_r,   // 0010
                    lhs.clone(),    // 0011
                    !lhs & rhs_r,   // 0100
                    rhs_r.clone(),  // 0101
                    lhs ^ rhs_r,    // 0110
                    lhs | rhs_r,    // 0111
                    !lhs & !rhs_r,  // 1000
                    !(lhs ^ rhs_r), // 1001
                    !rhs_r,         // 1010
                    lhs | !rhs_r,   // 1011
                    !lhs,           // 1100
                    !lhs | rhs_r,   // 1101
                    !lhs | !rhs_r,  // 1110
                    lhs | !lhs,     // 1111
                ];
                (actual == expected).then(|| ()).ok_or(i)?;

                let actual_len = actual.iter().map(|b| b.len());
                let actual_count = actual.iter().map(|b| b.indices(..).count());
                let expected_len = expected.iter().map(|b| b.len());
                actual_len.eq(expected_len.clone()).then(|| ()).ok_or(i)?;
                actual_count.eq(expected_len).then(|| ()).ok_or(i)?;
            }

            Ok(())
        }

        assert_eq!(test_internal(&bs0, &bs0), Ok(()));
        assert_eq!(test_internal(&bs0, &bs1), Ok(()));
        assert_eq!(test_internal(&bs0, &bs2), Ok(()));
        assert_eq!(test_internal(&bs0, &bs3), Ok(()));

        assert_eq!(test_internal(&bs1, &bs0), Ok(()));
        assert_eq!(test_internal(&bs1, &bs1), Ok(()));
        assert_eq!(test_internal(&bs1, &bs2), Ok(()));
        assert_eq!(test_internal(&bs1, &bs3), Ok(()));

        assert_eq!(test_internal(&bs2, &bs0), Ok(()));
        assert_eq!(test_internal(&bs2, &bs1), Ok(()));
        assert_eq!(test_internal(&bs2, &bs2), Ok(()));
        assert_eq!(test_internal(&bs2, &bs3), Ok(()));

        assert_eq!(test_internal(&bs3, &bs0), Ok(()));
        assert_eq!(test_internal(&bs3, &bs1), Ok(()));
        assert_eq!(test_internal(&bs3, &bs2), Ok(()));
        assert_eq!(test_internal(&bs3, &bs3), Ok(()));
    }
}
