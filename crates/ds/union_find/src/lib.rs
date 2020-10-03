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
        let (su, sv) = (self.buf.borrow()[u], self.buf.borrow()[v]);
        match (su, sv) {
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
        let mut bu = self.buf.borrow()[u];
        while let Item::Parent(pu) = bu {
            let tmp = pu;
            self.buf.borrow_mut()[u] = Item::Parent(res);
            u = tmp;
            bu = self.buf.borrow()[u];
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

#[cfg(test)]
mod tests {
    use crate::DisjointSet;
    use crate::UnionFind;
    #[test]
    fn test() {
        let n = 4;
        let mut uf = UnionFind::new(n);
        assert!(!uf.equiv(1, 3));
        assert_eq!(uf.count(1), 1);
        assert_eq!(uf.count(3), 1);
        uf.unite(1, 3);
        assert!(uf.equiv(1, 3));
        assert_eq!(uf.count(1), 2);
        assert_eq!(uf.count(3), 2);
        uf.unite(2, 3);
        assert!(uf.equiv(2, 3));
        assert!(uf.equiv(1, 2));
        assert!(!uf.equiv(1, 0));
    }
}
