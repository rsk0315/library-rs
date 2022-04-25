//! 全方位木 DP。

use std::collections::VecDeque;

/// 全方位木 DP。
///
/// 木の catamorphism。
/// 各頂点を根としたときのものをまとめて求める。
///
/// 子での値を辺の値を使って `map` したものたちを、`fold` するのを繰り返す。
/// 式とかを書く。`todo!()`
///
/// 前処理パートは `map` `fold` に依らないので使い回しできる。
///
/// # Examples
/// ```
/// use nekolib::graph::TreeCata;
///
/// let g = vec![
///     vec![(1, ()), (2, ())],
///     vec![(0, ()), (3, ()), (4, ()), (5, ())],
///     vec![(0, ())],
///     vec![(1, ())],
///     vec![(1, ())],
///     vec![(1, ())],
/// ];
///
/// //      0 -- 2
/// //      |
/// // 4 -- 1 -- 3
/// //      |
/// //      5
///
/// let tc: TreeCata<_> = g.into();
///
/// // max distance
/// let empty = 0;
/// let map = |&x: &usize, _: &()| x + 1;
/// let fold = |&x: &usize, &y: &usize| x.max(y);
/// assert_eq!(tc.each_root(empty, map, fold), [2, 2, 3, 3, 3, 3]);
///
/// // sum of distance
/// let empty = (0, 0);
/// let map = |&(d, c): &(usize, usize), _: &()| (d + c + 1, c + 1);
/// let fold =
///     |&x: &(usize, usize), &y: &(usize, usize)| (x.0 + y.0, x.1 + y.1);
/// assert_eq!(
///     tc.each_root(empty, map, fold)
///         .into_iter()
///         .map(|x| x.0)
///         .collect::<Vec<_>>(),
///     [8, 6, 12, 10, 10, 10]
/// );
///
/// ```
///
/// ```
/// use nekolib::graph::TreeCata;
///
/// let g = vec![
///     vec![(1, 0), (2, 0)],
///     vec![(0, 1), (3, 1), (4, 1), (5, 1)],
///     vec![(0, 2)],
///     vec![(1, 3)],
///     vec![(1, 4)],
///     vec![(1, 5)],
/// ];
///
/// let tc: TreeCata<_> = g.into();
///
/// let empty = "".to_owned();
/// let map = |x: &String, c: &usize| {
///     if x == "" { format!("{}: []", c) } else { format!("{}: [{}]", c, x) }
/// };
/// let fold = |x: &String, y: &String| {
///     if x == "" && y == "" {
///         "".to_owned()
///     } else if x != "" && y != "" {
///         format!("{}, {}", x, y)
///     } else {
///         format!("{}{}", x, y)
///     }
/// };
///
/// let actual = tc
///     .each_root(empty, map, fold)
///     .into_iter()
///     .enumerate()
///     .map(|(i, x)| format!("{}: [{}]", i, x))
///     .collect::<Vec<_>>();
///
/// assert_eq!(
///     actual,
///     [
///         "0: [1: [3: [], 4: [], 5: []], 2: []]",
///         "1: [0: [2: []], 3: [], 4: [], 5: []]",
///         "2: [0: [1: [3: [], 4: [], 5: []]]]",
///         "3: [1: [0: [2: []], 4: [], 5: []]]",
///         "4: [1: [0: [2: []], 3: [], 5: []]]",
///         "5: [1: [0: [2: []], 3: [], 4: []]]",
///     ]
/// );
///
/// let empty = "".to_owned();
/// let map = |x: &String, c: &usize| format!("({} {} )", x, c);
/// let fold = |x: &String, y: &String| format!("{}{}", x, y);
///
/// assert_eq!(tc.each_root(empty, map, fold), [
///     "(( 3 )( 4 )( 5 ) 1 )( 2 )",
///     "(( 2 ) 0 )( 3 )( 4 )( 5 )",
///     "((( 3 )( 4 )( 5 ) 1 ) 0 )",
///     "((( 2 ) 0 )( 4 )( 5 ) 1 )",
///     "((( 2 ) 0 )( 3 )( 5 ) 1 )",
///     "((( 2 ) 0 )( 3 )( 4 ) 1 )",
/// ]);
/// ```
///
/// # References
/// - <https://qiita.com/Kiri8128/items/a011c90d25911bdb3ed3>
///     - Efficient and easy 全方位木 DP。
/// - <https://fsharpforfunandprofit.com/posts/recursive-types-and-folds-1b/>
///     - catamorphism の話が載っている。
pub struct TreeCata<T> {
    par: Vec<Option<(usize, T)>>,
    order: Vec<usize>,
    child: Vec<Vec<(usize, T)>>,
    bound: Vec<usize>,
}

impl<T> From<Vec<Vec<(usize, T)>>> for TreeCata<T> {
    fn from(mut g: Vec<Vec<(usize, T)>>) -> Self {
        let n = g.len();
        let mut par: Vec<_> = (0..n).map(|_| None).collect();
        let mut q: VecDeque<_> = vec![0].into();
        let mut order = vec![];
        let mut child: Vec<_> = (0..n).map(|_| vec![]).collect();
        let mut bound = vec![n; n];

        while let Some(v) = q.pop_front() {
            order.push(v);
            let gv = std::mem::take(&mut g[v]);
            let mut left = true;
            for (nv, w) in gv {
                if nv == 0 || par[nv].is_some() {
                    par[v] = Some((nv, w));
                    left = false;
                } else {
                    if !left && bound[v] == n {
                        bound[v] = nv;
                    }
                    child[v].push((nv, w));
                    q.push_back(nv);
                }
            }
        }

        Self { par, order, child, bound }
    }
}

impl<T> TreeCata<T> {
    pub fn each_root<U: Clone>(
        &self,
        empty: U,
        mut map: impl FnMut(&U, &T) -> U,
        mut fold: impl FnMut(&U, &U) -> U,
    ) -> Vec<U> {
        let n = self.child.len();
        if n == 0 {
            return vec![];
        }

        let mut ascl: Vec<_> = vec![empty.clone(); n];
        let mut ascr: Vec<_> = vec![empty.clone(); n];
        let mut dp: Vec<_> = vec![empty.clone(); n];
        let mut right: Vec<_> = self.bound.iter().map(|&bi| bi < n).collect();
        for &i in self.order[1..].iter().rev() {
            dp[i] = fold(&ascl[i], &ascr[i]);
            let &(p, ref x) = self.par[i].as_ref().unwrap();
            if right[p] {
                ascr[p] = fold(&map(&dp[i], x), &ascr[p]);
                right[p] = self.bound[p] != i;
            } else {
                ascl[p] = fold(&map(&dp[i], x), &ascl[p]);
            }
        }
        dp[0] = fold(&ascl[0], &ascr[0]);

        let mut desc: Vec<_> = vec![empty.clone(); n];
        for &i in &self.order {
            let mut ac = desc[i].clone();
            for &(j, _) in &self.child[i] {
                let x = &self.par[j].as_ref().unwrap().1;
                desc[j] = ac.clone();
                ac = fold(&ac, &map(&dp[j], x));
            }
            let mut ac = empty.clone();
            for &(j, ref x) in self.child[i].iter().rev() {
                desc[j] = map(&fold(&desc[j], &ac), x);
                let x = &self.par[j].as_ref().unwrap().1;
                ac = fold(&map(&dp[j], x), &ac);
                let tmp = fold(&desc[j], &ascr[j]);
                dp[j] = fold(&ascl[j], &tmp);
            }
        }
        dp
    }
}

#[test]
fn test_value() {
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

    let g = vec![
        vec![(1, 0), (2, 0)],
        vec![(0, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2)],
        vec![(1, 3)],
        vec![(1, 4)],
        vec![(1, 5)],
    ];
    let tree_cata: TreeCata<_> = g.into();

    // string representation
    let empty = "".to_owned();
    let map = |x: &String, c: &usize| {
        if x == "" { format!("{}: []", c) } else { format!("{}: [{}]", c, x) }
    };
    let fold = |x: &String, y: &String| {
        if x == "" && y == "" {
            "".to_owned()
        } else if x != "" && y != "" {
            format!("{}, {}", x, y)
        } else {
            format!("{}{}", x, y)
        }
    };

    let actual = tree_cata
        .each_root(empty, map, fold)
        .into_iter()
        .enumerate()
        .map(|(i, x)| format!("{}: [{}]", i, x))
        .collect::<Vec<_>>();

    let expected = [
        "0: [1: [3: [], 4: [], 5: []], 2: []]",
        "1: [0: [2: []], 3: [], 4: [], 5: []]",
        "2: [0: [1: [3: [], 4: [], 5: []]]]",
        "3: [1: [0: [2: []], 4: [], 5: []]]",
        "4: [1: [0: [2: []], 3: [], 5: []]]",
        "5: [1: [0: [2: []], 3: [], 4: []]]",
    ];
    assert_eq!(actual, expected);
}

#[test]
fn test_order() {
    let empty = || "".to_owned();
    let map = |x: &String, c: &usize| format!("({} {} )", x, c);
    let fold = |x: &String, y: &String| format!("{}{}", x, y);

    // leftmost
    let g = vec![
        vec![(1, 0), (2, 0)],
        vec![(0, 1), (3, 1), (4, 1), (5, 1)],
        vec![(0, 2)],
        vec![(1, 3)],
        vec![(1, 4)],
        vec![(1, 5)],
    ];

    let tree_cata: TreeCata<_> = g.into();
    assert_eq!(tree_cata.each_root(empty(), map, fold), [
        "(( 3 )( 4 )( 5 ) 1 )( 2 )",
        "(( 2 ) 0 )( 3 )( 4 )( 5 )",
        "((( 3 )( 4 )( 5 ) 1 ) 0 )",
        "((( 2 ) 0 )( 4 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 4 ) 1 )",
    ]);

    // inner (1)
    let g = vec![
        vec![(1, 0), (2, 0)],
        vec![(3, 1), (0, 1), (4, 1), (5, 1)],
        vec![(0, 2)],
        vec![(1, 3)],
        vec![(1, 4)],
        vec![(1, 5)],
    ];

    let tree_cata: TreeCata<_> = g.into();
    assert_eq!(tree_cata.each_root(empty(), map, fold), [
        "(( 3 )( 4 )( 5 ) 1 )( 2 )",
        "( 3 )(( 2 ) 0 )( 4 )( 5 )",
        "((( 3 )( 4 )( 5 ) 1 ) 0 )",
        "((( 2 ) 0 )( 4 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 4 ) 1 )",
    ]);

    // inner (2)
    let g = vec![
        vec![(1, 0), (2, 0)],
        vec![(3, 1), (4, 1), (0, 1), (5, 1)],
        vec![(0, 2)],
        vec![(1, 3)],
        vec![(1, 4)],
        vec![(1, 5)],
    ];

    let tree_cata: TreeCata<_> = g.into();
    assert_eq!(tree_cata.each_root(empty(), map, fold), [
        "(( 3 )( 4 )( 5 ) 1 )( 2 )",
        "( 3 )( 4 )(( 2 ) 0 )( 5 )",
        "((( 3 )( 4 )( 5 ) 1 ) 0 )",
        "((( 2 ) 0 )( 4 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 4 ) 1 )",
    ]);

    // rightmost
    let g = vec![
        vec![(1, 0), (2, 0)],
        vec![(3, 1), (4, 1), (5, 1), (0, 1)],
        vec![(0, 2)],
        vec![(1, 3)],
        vec![(1, 4)],
        vec![(1, 5)],
    ];

    let tree_cata: TreeCata<_> = g.into();
    assert_eq!(tree_cata.each_root(empty(), map, fold), [
        "(( 3 )( 4 )( 5 ) 1 )( 2 )",
        "( 3 )( 4 )( 5 )(( 2 ) 0 )",
        "((( 3 )( 4 )( 5 ) 1 ) 0 )",
        "((( 2 ) 0 )( 4 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 5 ) 1 )",
        "((( 2 ) 0 )( 3 )( 4 ) 1 )",
    ]);
}
