use std::cell::RefCell;

use disjoint_set::DisjointSet;

#[derive(Clone, Copy)]
enum Item {
    Parent(usize),
    Size(usize),
}

pub struct UnionFind {
    n: usize,
    buf: RefCell<Vec<Item>>,
}

impl DisjointSet for UnionFind {
    fn new(n: usize) -> Self {
        Self {
            n,
            buf: RefCell::new(vec![Item::Size(1); n]),
        }
    }
    fn len(&self) -> usize {
        self.n
    }
    fn unite(&mut self, u: usize, v: usize) -> bool {
        let u = self.repr(u);
        let v = self.repr(v);
        if u == v {
            return false;
        }
        match (self.buf.borrow()[u], self.buf.borrow()[v]) {
            (Item::Size(su), Item::Size(sv)) => {
                let (child, par) = if su < sv { (u, v) } else { (v, u) };
                self.buf.borrow_mut()[par] = Item::Size(su + sv);
                self.buf.borrow_mut()[child] = Item::Parent(par);
            }
            _ => unreachable!(),
        }
        true
    }
    fn repr(&self, mut u: usize) -> usize {
        let mut res = u;
        while let Item::Parent(v) = self.buf.borrow()[res] {
            res = v;
        }
        while let Item::Parent(v) = self.buf.borrow()[u] {
            let tmp = v;
            self.buf.borrow_mut()[u] = Item::Parent(res);
            u = tmp;
        }
        res
    }
    fn count(&self, u: usize) -> usize {
        let u = self.repr(u);
        if let Item::Size(res) = self.buf.borrow()[u] {
            res
        } else {
            unreachable!()
        }
    }
}
