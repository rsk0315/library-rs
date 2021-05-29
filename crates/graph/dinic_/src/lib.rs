//! 最大流。

use std::cell::RefCell;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::ops::{AddAssign, SubAssign};
use std::rc::Rc;

use std::fmt::Debug;

fn main() {
    let es = vec![
        // (from, to, capacity)
        (0, 1, 10),
        (0, 2, 10),
        (1, 2, 2),
        (1, 3, 4),
        (1, 4, 8),
        (2, 4, 9),
        (3, 5, 10),
        (4, 3, 6),
        (4, 5, 10),
    ];
    let g = {
        let mut g = vec![vec![]; 6]; // [from]: [(to, capacity, rev), ...]
        for (from, to, capacity) in es {
            let from_len = g[from].len();
            let to_len = g[to].len();
            g[from].push((to, Rc::new(RefCell::new(capacity)), to_len));
            g[to].push((from, Rc::new(RefCell::new(0)), from_len));
        }
        g
    };

    let index = |&v: &usize| v;
    let delta = |&v: &usize| g[v].iter().map(|(nv, w, r)| (*nv, w.clone(), *r));
    let rev = |&nv: &usize, &r: &usize| g[nv][r].1.clone();
    let flow = dinic(6, 0, 5, 0..6, 0, index, delta, rev);
    println!("{}", flow);

    {
        // https://gist.github.com/MiSawa/47b1d99c372daffb6891662db1a2b686
        let n = 100;
        let (s, a, b, c, t) = (0, 1, 2, 3, 4);
        let mut uv = (5, 6);
        let mut es = vec![
            (s, a, 1),
            (s, b, 2),
            (b, a, 2),
            (c, t, 2),
            (a, uv.0, 3),
            (a, uv.1, 3),
        ];
        while uv.1 + 2 < n {
            let nuv = (uv.0 + 2, uv.1 + 2);
            for &x in &[uv.0, uv.1] {
                for &y in &[nuv.0, nuv.1] {
                    es.push((x, y, 3));
                }
            }
            uv = nuv;
        }
        es.push((uv.0, c, 3));
        es.push((uv.1, c, 3));

        let g = {
            let mut g = vec![vec![]; n];
            for (from, to, capacity) in es {
                let from_len = g[from].len();
                let to_len = g[to].len();
                g[from].push((to, Rc::new(RefCell::new(capacity)), to_len));
                g[to].push((from, Rc::new(RefCell::new(0)), from_len));
            }
            g
        };

        let index = |&v: &usize| v;
        let delta =
            |&v: &usize| g[v].iter().map(|&(nv, ref w, r)| (nv, w.clone(), r));
        let rev = |&nv: &usize, &r: &usize| g[nv][r].1.clone();
        let flow = dinic(n, s, t, 0..n, 0, index, delta, rev);
        println!("{}", flow);
    }
}

/// 最大流。
pub fn dinic<V, W, R, F>(
    n: usize,
    s: V,
    t: V,
    vs: impl Iterator<Item = V> + Clone,
    zero: W,
    index: impl Fn(&V) -> usize + Copy,
    delta: impl Fn(&V) -> F + Copy,
    rev: impl Fn(&V, &R) -> Rc<RefCell<W>> + Copy,
) -> W
where
    V: Clone,
    W: Ord + Clone + AddAssign + SubAssign,
    R: Clone,
    F: Iterator<Item = (V, Rc<RefCell<W>>, R)>,
    V: Debug,
    W: Debug,
{
    let mut res = zero.clone();
    loop {
        let level = dual(n, s.clone(), zero.clone(), index, delta);
        if level[index(&t)] == n {
            break;
        }
        let iter: Vec<_> = vs
            .clone()
            .map(|v| Rc::new(RefCell::new(delta(&v).peekable())))
            .collect();
        loop {
            match primal(&s, &t, zero.clone(), &level, index, rev, &iter) {
                Some(f) => res += f,
                None => break,
            }
        }
    }
    res
}

fn dual<V, W, R, F>(
    n: usize,
    s: V,
    zero: W,
    index: impl Fn(&V) -> usize,
    delta: impl Fn(&V) -> F,
) -> Vec<usize>
where
    V: Clone,
    W: Ord + Clone + AddAssign + SubAssign,
    R: Clone,
    F: Iterator<Item = (V, Rc<RefCell<W>>, R)>,
    V: Debug,
    W: Debug,
{
    let mut level = vec![n; n];
    let mut q = VecDeque::new();
    level[index(&s)] = 0;
    q.push_back(s);
    while let Some(v) = q.pop_front() {
        let i = index(&v);
        for (nv, w, _) in delta(&v) {
            let ni = index(&nv);
            if *w.borrow() > zero && level[ni] == n {
                level[ni] = level[i] + 1;
                q.push_back(nv);
            }
        }
    }
    level
}

fn primal<V, W, R, I>(
    s: &V,
    t: &V,
    zero: W,
    level: &[usize],
    index: impl Fn(&V) -> usize,
    rev: impl Fn(&V, &R) -> Rc<RefCell<W>>,
    iter: &[Rc<RefCell<Peekable<I>>>],
) -> Option<W>
where
    V: Clone,
    W: Ord + Clone + AddAssign + SubAssign,
    R: Clone,
    I: Iterator<Item = (V, Rc<RefCell<W>>, R)>,
    V: Debug,
    W: Debug,
{
    let ti = index(t);
    let mut es = vec![];
    let mut vis = vec![index(s)];
    'find_path: while let Some(&vi) = vis.last() {
        if vi == ti {
            break;
        }
        loop {
            if let Some((nv, w, r)) = iter[vi].borrow_mut().peek() {
                let nvi = index(&nv);
                if *w.borrow() > zero && level[vi] < level[nvi] {
                    es.push((w.clone(), nv.clone(), r.clone()));
                    vis.push(nvi);
                    continue 'find_path;
                }
            } else {
                break;
            }

            iter[vi].borrow_mut().next();
        }
        es.pop();
        vis.pop();
        if let Some(&vi) = vis.last() {
            iter[vi].borrow_mut().next();
        }
    }

    if es.is_empty() {
        return None;
    }

    let f = es.iter().map(|(e, _, _)| e.borrow().clone()).min().unwrap();

    for (w, nv, r) in es {
        *w.borrow_mut() -= f.clone();
        *rev(&nv, &r).borrow_mut() += f.clone();
    }

    Some(f)
}
