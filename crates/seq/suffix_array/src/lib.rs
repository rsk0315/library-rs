//! 接尾辞配列。
//!
//! See [CS166](http://web.stanford.edu/class/archive/cs/cs166/cs166.1206/lectures/04/Small04.pdf).

use std::cmp::Ordering::{Equal, Less};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::ops::Index;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuffixArray<T: Ord> {
    buf: Vec<T>,
    sa: Vec<usize>,
}

impl<T: Clone + Ord, S: AsRef<[T]>> From<S> for SuffixArray<T> {
    fn from(buf: S) -> Self {
        let buf: Vec<_> = buf.as_ref().to_vec();
        let buf_usize = compress(&buf);
        let sa = sa_is(&buf_usize);
        Self { buf, sa }
    }
}

fn compress<T: Clone + Ord>(buf: &[T]) -> Vec<usize> {
    let enc: BTreeSet<_> = buf.iter().cloned().collect();
    let enc: BTreeMap<_, _> =
        enc.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    buf.iter()
        .map(|x| enc[x] + 1)
        .chain(std::iter::once(0)) // for '$'
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LsType {
    LType,
    SType(bool), // true if leftmost S-type
}
use LsType::*;

fn count_freq(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().for_each(|&x| res[x] += 1);
    res
}

fn inv_perm(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().enumerate().for_each(|(i, &x)| res[x] = i);
    res
}

fn ls_classify(buf: &[usize]) -> Vec<LsType> {
    let mut res = vec![SType(false); buf.len()];
    for i in (0..buf.len() - 1).rev() {
        res[i] = match buf[i].cmp(&buf[i + 1]) {
            Less => SType(false),
            Equal if res[i + 1] == SType(false) => SType(false),
            _ => LType,
        };
    }
    for i in 1..buf.len() {
        if let (LType, SType(_)) = (res[i - 1], res[i]) {
            res[i] = SType(true);
        }
    }
    res
}

fn init_offset(count: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut tail = count.to_vec();
    for i in 1..count.len() {
        tail[i] += tail[i - 1];
    }
    let head = std::iter::once(0)
        .chain(tail.iter().take(tail.len() - 1).cloned())
        .collect();
    (head, tail)
}

fn induce(
    buf: &[usize],
    sa: &mut [Option<usize>],
    head: &mut [usize],
    tail: &mut [usize],
    count: &[usize],
    ls: &[LsType],
) {
    for i in 0..sa.len() {
        match sa[i] {
            Some(j) if j > 0 && ls[j - 1] == LType => {
                sa[head[buf[j - 1]]] = Some(j - 1);
                head[buf[j - 1]] += 1;
            }
            _ => {}
        }
    }
    tail.copy_from_slice(count);
    (1..tail.len()).for_each(|i| tail[i] += tail[i - 1]);
    for i in (1..count.len()).rev() {
        match sa[i] {
            Some(j) if j > 0 && ls[j - 1] != LType => {
                tail[buf[j - 1]] -= 1;
                sa[tail[buf[j - 1]]] = Some(j - 1);
            }
            _ => {}
        }
    }
}

fn reduce(buf: &[usize], lms: Vec<usize>, ls: &[LsType]) -> Vec<usize> {
    if lms.len() <= 1 {
        return vec![0; lms.len()];
    }

    let mut map = vec![0; buf.len()]; // map[lms[0]] = 0
    map[lms[1]] = 1;
    let mut x = 1;
    for i in 2..lms.len() {
        let equiv = buf[lms[i]] == buf[lms[i - 1]]
            && (lms[i] + 1..)
                .zip(lms[i - 1] + 1..)
                .find_map(|(i0, i1)| {
                    if (ls[i0], ls[i1]) == (SType(true), SType(true)) {
                        Some(true)
                    } else if ls[i0] != ls[i1] || buf[i0] == buf[i1] {
                        Some(false)
                    } else {
                        None
                    }
                })
                .unwrap();

        if !equiv {
            x += 1;
        }
        map[lms[i]] = x;
    }

    (0..buf.len())
        .filter_map(|i| match ls[i] {
            SType(true) => Some(map[i]),
            _ => None,
        })
        .collect()
}

fn sa_is(buf: &[usize]) -> Vec<usize> {
    let len = buf.len();
    let count = count_freq(buf);
    if count.iter().all(|&x| x == 1) {
        return inv_perm(buf);
    }

    let ls = ls_classify(buf);
    let mut sa = vec![None; len];
    let (mut head, mut tail) = init_offset(&count);
    for i in (1..len).rev().filter(|&i| ls[i] == SType(true)) {
        tail[buf[i]] -= 1;
        sa[tail[buf[i]]] = Some(i);
    }
    induce(buf, &mut sa, &mut head, &mut tail, &count, &ls);

    let lms: Vec<_> = sa
        .into_iter()
        .map(std::option::Option::unwrap)
        .filter_map(|i| if ls[i] == SType(true) { Some(i) } else { None })
        .collect(); // in lexicographic order
    let rs_sa = sa_is(&reduce(buf, lms, &ls));

    let lms: Vec<_> = (0..len)
        .filter_map(|i| if ls[i] == SType(true) { Some(i) } else { None })
        .collect(); // in appearing order

    let (mut head, mut tail) = init_offset(&count);
    let mut sa = vec![None; len];
    for i in rs_sa.into_iter().rev() {
        let j = lms[i];
        tail[buf[j]] -= 1;
        sa[tail[buf[j]]] = Some(j);
    }
    induce(buf, &mut sa, &mut head, &mut tail, &count, &ls);

    sa.into_iter().map(std::option::Option::unwrap).collect()
}

impl<T: Ord> Index<usize> for SuffixArray<T> {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        &self.sa[i]
    }
}

impl<T: Ord> From<SuffixArray<T>> for Vec<usize> {
    fn from(sa: SuffixArray<T>) -> Self {
        sa.sa
    }
}
