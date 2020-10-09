use std::ops::Range;

use crate::jury;
use crate::test_set::*;

use jury::aoj_0425::{Query, Response};

use range_hash::RangeHash;

pub struct Aoj0425<R> {
    _r: std::marker::PhantomData<R>,
}

impl<R> Solver for Aoj0425<R>
where
    R: RangeHash<Input = Query, Output = Response>
        + From<(usize, Vec<(usize, usize)>)>,
{
    type Jury = jury::Aoj0425;
    fn solve(
        (n, ab, qs): (usize, Vec<(usize, usize)>, Vec<(Range<usize>, Query)>),
    ) -> Vec<Response> {
        let mut rh: R = (n, ab).into();
        rh.batch_query(qs, Some(224))
    }
}

pub struct NekoAoj0425 {
    n: usize,
    l: usize,
    r: usize,
    ab: Vec<(usize, usize)>,
    p: Vec<usize>,
    q: Vec<usize>,
}

impl NekoAoj0425 {
    fn swap(&mut self, a: usize, b: usize) {
        self.q.swap(self.p[a], self.p[b]);
        self.p.swap(a, b);
    }
}

impl From<(usize, Vec<(usize, usize)>)> for NekoAoj0425 {
    fn from((n, ab): (usize, Vec<(usize, usize)>)) -> Self {
        Self {
            n,
            ab,
            l: 0,
            r: 0,
            p: (0..n).collect(),
            q: (0..n).collect(),
        }
    }
}

impl RangeHash for NekoAoj0425 {
    type Input = jury::aoj_0425::Query;
    type Output = jury::aoj_0425::Response;
    fn start(&self) -> usize {
        self.l
    }
    fn end(&self) -> usize {
        self.r
    }
    fn full_len(&self) -> usize {
        self.n
    }
    fn reset(&mut self) {
        self.l = 0;
        self.r = 0;
        self.p = (0..self.n).collect();
        self.q = (0..self.n).collect();
    }
    fn expand_back(&mut self) {
        let (a, b) = self.ab[self.r];
        self.r += 1;
        self.swap(a, b);
    }
    fn expand_front(&mut self) {
        self.l -= 1;
        let (a, b) = self.ab[self.l];
        self.swap(self.q[a], self.q[b]);
    }
    fn shrink_back(&mut self) {
        self.r -= 1;
        let (a, b) = self.ab[self.r];
        self.swap(a, b);
    }
    fn shrink_front(&mut self) {
        let (a, b) = self.ab[self.l];
        self.l += 1;
        self.swap(self.q[a], self.q[b]);
    }
    fn hash(&self, q: Self::Input) -> Self::Output {
        use jury::aoj_0425::{Query, Response};
        match q {
            Query::Type1(x) => Response::Type1(self.p[x]),
            Query::Type2(x) => Response::Type2(self.q[x]),
        }
    }
}
