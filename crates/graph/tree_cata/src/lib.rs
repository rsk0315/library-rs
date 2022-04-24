use std::collections::VecDeque;

pub struct TreeCata<T> {
    p: Vec<Option<(usize, T)>>,
    r: Vec<usize>,
    x: Vec<Vec<(usize, T)>>,
}

impl<T> From<Vec<Vec<(usize, T)>>> for TreeCata<T> {
    fn from(mut g: Vec<Vec<(usize, T)>>) -> Self {
        let n = g.len();
        let mut p: Vec<_> = (0..n).map(|_| None).collect();
        let mut q: VecDeque<_> = vec![0].into();
        let mut r = vec![];
        let mut x: Vec<_> = (0..n).map(|_| vec![]).collect();

        while let Some(v) = q.pop_front() {
            r.push(v);
            let gv = std::mem::take(&mut g[v]);
            for (nv, w) in gv {
                if nv == 0 || p[nv].is_some() {
                    p[v] = Some((nv, w));
                } else {
                    x[v].push((nv, w));
                    q.push_back(nv);
                }
            }
        }

        Self { p, r, x }
    }
}

impl<T> TreeCata<T> {
    pub fn each_root<U: Clone>(
        &self,
        empty: U,
        mut map: impl FnMut(&U, &T) -> U,
        mut fold: impl FnMut(&U, &U) -> U,
    ) -> Vec<U> {
        let n = self.x.len();
        if n == 0 {
            return vec![empty];
        }

        let mut me: Vec<_> = vec![empty.clone(); n];
        let mut xx: Vec<_> = vec![empty.clone(); n];
        for &i in self.r[1..].iter().rev() {
            xx[i] = me[i].clone();
            let &(p, ref x) = self.p[i].as_ref().unwrap();
            me[p] = fold(&map(&xx[i], x), &me[p]);
        }
        let r0 = self.r[0];
        xx[r0] = me[r0].clone();

        let mut td: Vec<_> = vec![empty.clone(); n];
        for &i in &self.r {
            let mut ac = td[i].clone();
            for &(j, _) in &self.x[i] {
                let x = &self.p[j].as_ref().unwrap().1;
                td[j] = ac.clone();
                ac = fold(&ac, &map(&xx[j], x));
            }
            let mut ac = empty.clone();
            for &(j, ref x) in self.x[i].iter().rev() {
                td[j] = map(&fold(&td[j], &ac), x);
                let x = &self.p[j].as_ref().unwrap().1;
                ac = fold(&map(&xx[j], x), &ac);
                xx[j] = fold(&td[j], &me[j]);
            }
        }
        xx
    }
}

#[test]
fn test() {
    let n = 6;
    let es = vec![(0, 1), (0, 2), (1, 3), (1, 4), (1, 5)];
    let g = {
        let mut g = vec![vec![]; n];
        for &(u, v) in &es {
            g[u].push((v, ()));
            g[v].push((u, ()));
        }
        g
    };
    let tree_cata: TreeCata<_> = g.into();

    // max distance
    let empty = 0;
    let map = |&x: &usize, _: &()| x + 1;
    let fold = |&x: &usize, &y: &usize| x.max(y);
    assert_eq!(tree_cata.each_root(empty, map, fold), [2, 2, 3, 3, 3, 3]);

    // sum of distance
    let empty = (0, 0);
    let map = |&(d, c): &(usize, usize), _: &()| (d + c + 1, c + 1);
    let fold =
        |&x: &(usize, usize), &y: &(usize, usize)| (x.0 + y.0, x.1 + y.1);
    assert_eq!(
        tree_cata
            .each_root(empty, map, fold)
            .into_iter()
            .map(|x| x.0)
            .collect::<Vec<_>>(),
        [8, 6, 12, 10, 10, 10]
    );

    let g = {
        let mut g = vec![vec![]; n];
        for &(u, v) in &es {
            g[u].push((v, u));
            g[v].push((u, v));
        }
        g
    };
    let tree_cata: TreeCata<_> = g.into();

    // string representation
    let empty = "".to_owned();
    let map = |x: &String, c: &usize| format!("({} {} )", x, c);
    let fold = |x: &String, y: &String| format!("{}{}", x, y);
    assert_eq!(tree_cata.each_root(empty, map, fold), [
        "(( 3 )( 4 )( 5 ) 1 )( 2 )",
        "(( 2 ) 0 )( 3 )( 4 )( 5 )",
        "((( 3 )( 4 )( 5 ) 1 ) 0 )",
        "((( 2 ) 0 )( 4 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 4 ) 1 )",
    ]);
}
