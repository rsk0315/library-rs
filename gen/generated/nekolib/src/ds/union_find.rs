//! union-find。

use super::super::traits::disjoint_set;

use std::cell::RefCell;

use disjoint_set::DisjointSet;

#[derive(Clone, Copy)]
enum Item {
    Parent(usize),
    Size(usize),
}

/// union-find。
///
/// # Complexity
/// |演算|時間計算量|
/// |---|---|
/// |`new`|$\\Theta(n)$|
/// |`unite`|amortized $O(\\alpha(n))$|
/// |`repr`|amortized $O(\\alpha(n))$|
/// |`equiv`|amortized $O(\\alpha(n))$|
/// |`count`|amortized $O(\\alpha(n))$|
/// |`subset`|$\\Theta(n)$|
///
/// より tight には、$n$ 要素 $m$ クエリのとき、$O(m\\cdot\\alpha(m, n)+n)$ 時間となる。
/// ただし、$\\alpha(m, n)$ は次のように定義される。
/// $$ \\alpha(m, n) = \\min\\{k\\in\\mathbb{N}\\mid J\_k(\\lfloor\\log\_2(n)\\rfloor)\\le 1+m/n\\}. $$
/// ここで、$g^\\diamond = (\\lceil{\\log\_2}\\rceil\\circ g)^\\star$ とし、$J\_0(r) =
/// \\lceil(r-1)/2\\rceil$, $J\_k(r)=J\_{k-1}^\\diamond(r)$ ($k\\gt 0$) である。
/// より直感的には、${\\underbrace{J\_0^{\\diamond\\diamond\\cdots\\diamond}}\_{k\\text{ many }\\diamond{\\text{s}}}}(\\lfloor\\log\_2(n)\\rfloor)$ が $1+m/n$ 以下になる最小の $k$ が $\\alpha(m, n)$ である。
///
///
/// ## Complexity analysis
///
/// 参考文献ふたつめの PDF の概略を書く。`todo!()`
///
/// これらより、$f(m, n, r)\\le (\\alpha(m, n)+2)\\cdot m+2n$ が言える。
///
/// Note: $\\alpha\'(m, n) = \\min\\{k\\in\\mathbb{N}\\mid \\alpha\_k(n)\\le 3\\}$ として
/// $\\alpha(m, n) = O(\\alpha\'(m, n))$ である。
///
///
/// # Examples
/// ```
/// use nekolib::traits::DisjointSet;
/// use nekolib::ds::UnionFind;
///
/// let mut uf = UnionFind::new(4);
/// assert!(!uf.equiv(0, 2));
/// uf.unite(0, 1);
/// uf.unite(1, 2);
/// assert!(uf.equiv(0, 2));
/// assert!(!uf.equiv(0, 3));
/// assert_eq!(uf.count(0), 3);
/// ```
///
/// # References
/// - <http://www.gabrielnivasch.org/fun/inverse-ackermann>
/// - <http://cgi.di.uoa.gr/~ewcg06/invited/Seidel.pdf>
#[derive(Clone)]
pub struct UnionFind {
    n: usize,
    buf: RefCell<Vec<Item>>,
}

impl DisjointSet for UnionFind {
    fn new(n: usize) -> Self {
        Self { n, buf: RefCell::new(vec![Item::Size(1); n]) }
    }
    fn len(&self) -> usize { self.n }
    fn unite(&mut self, u: usize, v: usize) -> bool {
        let u = self.repr(u);
        let v = self.repr(v);
        if u == v {
            return false;
        }
        let mut buf = self.buf.borrow_mut();
        let (su, sv) = (buf[u], buf[v]);
        match (su, sv) {
            (Item::Size(su), Item::Size(sv)) => {
                let (child, par) = if su < sv { (u, v) } else { (v, u) };
                buf[par] = Item::Size(su + sv);
                buf[child] = Item::Parent(par);
            }
            _ => unreachable!(),
        }
        true
    }
    fn repr(&self, mut u: usize) -> usize {
        let mut res = u;
        let mut buf = self.buf.borrow_mut();
        while let Item::Parent(v) = buf[res] {
            res = v;
        }
        let mut bu = buf[u];
        while let Item::Parent(pu) = bu {
            let tmp = pu;
            buf[u] = Item::Parent(res);
            u = tmp;
            bu = buf[u];
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
