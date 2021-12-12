//! ポテンシャルつき union-find。

use super::super::traits::binop;
use super::super::traits::potential_function;

use std::cell::RefCell;

use binop::{CommutativeGroup, Magma};
use potential_function::PotentialFunction;

#[derive(Clone, Copy)]
enum Item {
    Parent(usize),
    Size(usize),
}

/// ポテンシャルつき union-find。
///
/// # Idea
/// 通常の union-find に加え、配列 `pot` を管理する。
/// 親ノード `par` と子ノード `child` に対して、`pot[child]` には
/// `phi(child) - phi(par)` を持つようにする。
///
/// 代表元を探してパスを縮約する際、ポテンシャル差の更新を適切に行う。
#[derive(Clone)]
pub struct PotentializedUnionFind<G: CommutativeGroup>
where
    <G as Magma>::Set: Clone,
{
    n: usize,
    buf: RefCell<Vec<Item>>,
    pot: RefCell<Vec<<G as Magma>::Set>>,
    cgroup: G,
}

impl<G: CommutativeGroup> PotentialFunction for PotentializedUnionFind<G>
where
    <G as Magma>::Set: Clone,
{
    type Item = G;
    fn new(n: usize, cgroup: G) -> Self {
        Self {
            n,
            buf: RefCell::new(vec![Item::Size(1); n]),
            pot: RefCell::new(vec![cgroup.id(); n]),
            cgroup,
        }
    }

    fn len(&self) -> usize { self.n }

    fn relate(&mut self, u: usize, v: usize, w: G::Set) -> Option<bool> {
        let ru = self.repr(u);
        let rv = self.repr(v);
        let mut buf = self.buf.borrow_mut();
        let mut pot = self.pot.borrow_mut();
        // w += p[v] - p[u];
        let w = self.cgroup.op(
            w,
            self.cgroup.op(pot[v].clone(), self.cgroup.recip(pot[u].clone())),
        );
        if ru == rv {
            return if w == self.cgroup.id() { Some(false) } else { None };
        }

        let (su, sv) = match (buf[ru], buf[rv]) {
            (Item::Size(su), Item::Size(sv)) => (su, sv),
            _ => unreachable!(),
        };

        let (child, par, d) =
            if su < sv { (ru, rv, w) } else { (rv, ru, self.cgroup.recip(w)) };
        buf[par] = Item::Size(su + sv);
        buf[child] = Item::Parent(par);
        pot[child] = d;
        Some(true)
    }

    fn diff(&self, u: usize, v: usize) -> Option<G::Set> {
        if self.repr(u) != self.repr(v) {
            return None;
        }
        let pot = self.pot.borrow();
        Some(self.cgroup.op(pot[u].clone(), self.cgroup.recip(pot[v].clone())))
    }
}

impl<G: CommutativeGroup> PotentializedUnionFind<G>
where
    <G as Magma>::Set: Clone,
{
    pub fn with_len(n: usize) -> Self
    where
        G: Default,
    {
        Self::new(n, G::default())
    }

    fn repr(&self, mut u: usize) -> usize {
        let mut res = u;
        let mut buf = self.buf.borrow_mut();
        let mut pot = self.pot.borrow_mut();
        let mut p = self.cgroup.id();
        while let Item::Parent(v) = buf[res] {
            p = self.cgroup.op(p.clone(), pot[res].clone());
            res = v;
        }
        let mut bu = buf[u];
        while let Item::Parent(pu) = bu {
            buf[u] = Item::Parent(res);
            let tmp = p.clone();
            p = self.cgroup.op(p.clone(), self.cgroup.recip(pot[u].clone()));
            pot[u] = tmp;
            u = pu;
            bu = buf[u];
        }
        res
    }
}
