use crate::jury;
use crate::test_set::{Jury, Solver};

use jury::aoj_0425::{Query, Response};

use elastic_slice::{
    ElasticSlice, ExpandBack, ExpandFront, ShrinkBack, ShrinkFront, SliceHash,
};
use mo::mo;

pub struct Aoj0425<S> {
    _s: std::marker::PhantomData<S>,
}

impl<S> Solver for Aoj0425<S>
where
    S: ElasticSlice
        + ExpandBack
        + ExpandFront
        + ShrinkBack
        + ShrinkFront
        + SliceHash<
            Salt = jury::aoj_0425::Query,
            Hashed = jury::aoj_0425::Response,
        > + From<(usize, Vec<(usize, usize)>)>,
{
    type Jury = jury::Aoj0425;
    fn solve((n, ab, qs): <Self::Jury as Jury>::Input) -> Vec<Response> {
        let rs: S = (n, ab).into();
        mo(rs, qs, Some(224))
    }
}

pub struct Neko {
    n: usize,
    l: usize,
    r: usize,
    ab: Vec<(usize, usize)>,
    p: Vec<usize>,
    q: Vec<usize>,
}

impl Neko {
    fn swap(&mut self, a: usize, b: usize) {
        self.q.swap(self.p[a], self.p[b]);
        self.p.swap(a, b);
    }
}

impl From<(usize, Vec<(usize, usize)>)> for Neko {
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

impl ElasticSlice for Neko {
    fn reset(&mut self) {
        self.l = 0;
        self.r = 0;
        self.p = (0..self.n).collect();
        self.q = (0..self.n).collect();
    }
    fn full_len(&self) -> usize {
        self.n
    }
    fn start(&self) -> usize {
        self.l
    }
    fn end(&self) -> usize {
        self.r
    }
}

impl ExpandBack for Neko {
    fn expand_back(&mut self) {
        let (a, b) = self.ab[self.r];
        self.r += 1;
        self.swap(a, b);
    }
}

impl ExpandFront for Neko {
    fn expand_front(&mut self) {
        self.l -= 1;
        let (a, b) = self.ab[self.l];
        self.swap(self.q[a], self.q[b]);
    }
}

impl ShrinkBack for Neko {
    fn shrink_back(&mut self) {
        self.r -= 1;
        let (a, b) = self.ab[self.r];
        self.swap(a, b);
    }
}

impl ShrinkFront for Neko {
    fn shrink_front(&mut self) {
        let (a, b) = self.ab[self.l];
        self.l += 1;
        self.swap(self.q[a], self.q[b]);
    }
}

impl SliceHash for Neko {
    type Salt = jury::aoj_0425::Query;
    type Hashed = jury::aoj_0425::Response;
    fn hash(&self, q: Self::Salt) -> Self::Hashed {
        match q {
            Query::Type1(x) => Response::Type1(self.p[x]),
            Query::Type2(x) => Response::Type2(self.q[x]),
        }
    }
}
