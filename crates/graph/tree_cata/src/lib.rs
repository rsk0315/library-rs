//! 全方位木 DP。

use std::collections::VecDeque;

/// 全方位木 DP。
///
/// 木の catamorphism。
/// 各頂点を根としたときのものをまとめて求める。
///
/// 二項演算 $\circ: S\\times S\\to S$ と $\star: S\\times T\\to S$ を考える。
/// $(S, \\circ)$ は単位元 $\\mathrm{id}\_{\\circ}$ を持つモノイドとする。
/// 木の各辺は一つずつ $T$ の値を持っているとし、辺 $(v, u)$ の値を $e\_{v, u}$
/// とする。
///
/// 頂点 $v$ が葉のとき、$f(v) = \\mathrm{id}\_{\\circ}$ とする[^1]。
/// そうでないとき、$v$ に隣接する頂点を順に
/// $\\langle u\_1, u\_2, \\dots, u\_k\\rangle$ とする。$v$ が根のとき、
/// $$ f(v) = (f\_v(u\_1)\\star e\_{u\_1, v})\\circ\\dots\\circ(f\_v(u\_k)\\star e\_{u\_k, v}) $$
/// で定める。ただし、$f\_v(u)$ は、「$v$ を取り除いた森において $u$ を含む木で
/// $u$ を根としたもの」における $f(u)$ とする。
///
/// [^1]: すなわち、頂点数 1 の木における $f$ の値が $\\mathrm{id}\_{\\circ}$ となる。
///
/// <img src="../../../../images/tree_cata.png" width="300" alt=""><img src="../../../../../images/tree_cata.png" width="300" alt="">
///
/// 上図グラフにおいて、頂点 $1$ に隣接する頂点のうち、$0$ が最後に来るものとすれば
/// $$ \\begin{aligned}
/// f(1) &= f\_0(1)\\circ(f\_1(0)\\star e\_{0, 1}) \\\\
/// &= f\_0(1) \\circ ((f\_0(2)\\star e\_{2, 0})\\star e\_{0, 1})
/// \\end{aligned} $$
/// のようになる。
///
/// このように定められる $f$ に対し、$f(0), f(1), \\dots, f(n-1)$ を求める。
///
/// # Idea
/// まず、根を $0$ として木をトポロジカルソートしておく。
/// これにより、ボトムアップの DP を単にループで行うことができ、$f(0)$
/// が求まる。次に、上で $f(1)$ を求めたときのように、トップダウンに DP
/// をしながら（ボトムアップの DP での結果を利用して）残りの頂点について求める。
///
/// ## Implementation notes
/// トポロジカルソートの前処理パートは、木が同じであれば $(\\circ, \\star)$
/// に依らないので使い回しできる。
///
/// 実装においては、$\\circ: S\\times S\\to S$ を `fold`、$\\mathrm{id}\_{\\circ}$
/// を `empty`、$\\star: S\\times T\\to S$ を `map` と呼んでいる。
///
/// `empty` は葉での値（頂点数 1 の木での値）と `fold` の単位元に対応している。
/// 葉の値を特別扱いしたいときは、セグ木に半群を乗せるときのように、
/// フラグを持たせれば対応できる（下記の root-leaf の距離の総和の例を参照）。
/// 全体の頂点数が 1 だったときの処理を最後に分ける必要があるので注意。
///
/// 各頂点における頂点の順序を気にした実装になっているため、「各頂点を根として
/// DFS したときの post-order で各頂点を並べ、その列に基数 $b$・法 $m$ の
/// rolling hash を適用したときの値を求めよ」といった問題も処理できるはず。
/// ハッシュ値 $h$ と、部分木サイズ $k$ に対して $(h, b^k\\bmod m)$
/// とかを管理すればよさそう。
///
/// # Complexity
/// $O(n)$ time.
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
/// // (sum of root-leaf distance, # of leaves)
/// let empty = ((0, 0), false);
/// let map = |&(x, inner): &((usize, usize), bool), _: &()| {
///     let (x1, x0) = if inner { x } else { (0, 1) };
///     ((x1 + 1 * x0, x0), true)
/// };
/// let fold = |&x: &((usize, usize), bool), &y: &((usize, usize), bool)| {
///     let (x1, x0) = x.0;
///     let (y1, y0) = y.0;
///     ((x1 + y1, x0 + y0), x.1 | y.1)
/// };
/// assert_eq!(
///     tc.each_root(empty, map, fold)
///         .into_iter()
///         .map(|x| if x.1 { x.0 } else { (0, 1) })
///         .collect::<Vec<_>>(),
///     [(7, 4), (5, 4), (9, 3), (7, 3), (7, 3), (7, 3)]
/// );
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
/// ## Applications
/// AtCoder における利用例。$(S, \\circ, \\mathrm{id}\_{\\circ}, T, \\star)$
/// の定義と、$\\langle f(0), f(1), \\dots, f(n-1)\\rangle$
/// を用いて答えを得る部分のみ載せている。
///
/// ```ignore
/// // typical90_am
/// let empty = (0, 0);
/// let map = |&x: &(usize, usize), _: &()| (x.0 + x.1 + 1, x.1 + 1);
/// let fold =
///     |&x: &(usize, usize), &y: &(usize, usize)| (x.0 + y.0, x.1 + y.1);
/// let res: usize =
///     tc.each_root(empty, map, fold).into_iter().map(|x| x.0).sum();
/// ```
/// ```ignore
/// // abc220_f
/// let empty = (0, 0);
/// let map = |&(x1, x0): &(usize, usize), _: &()| (x1 + x0 + 1, x0 + 1);
/// let fold = |&(x1, x0): &(usize, usize), &(y1, y0): &(usize, usize)| {
///     (x1 + y1, x0 + y0)
/// };
/// let res: Vec<_> =
///     tc.each_root(empty, map, fold).into_iter().map(|(x1, _)| x1).collect();
/// ```
/// ```ignore
/// // abc222_f
/// let empty = 0;
/// let map = |&x: &i64, &(d, w): &(i64, i64)| x.max(d) + w;
/// let fold = |&x: &i64, &y: &i64| x.max(y);
/// let res = tc.each_root(empty, map, fold);
/// ```
/// ```ignore
/// // abc223_g
/// let empty = false;
/// let map = |&x: &bool, _: &()| !x;
/// let fold = |&x: &bool, &y: &bool| x | y;
/// let res =
///     tc.each_root(empty, map, fold).into_iter().filter(|&x| !x).count();
/// ```
/// ```ignore
/// // s8pc_4_d
/// let empty = (0.0, 0);
/// let map = |&x: &(f64, usize), _: &()| (x.0 + 1.0, 1);
/// let fold = |&x: &(f64, usize), &y: &(f64, usize)| {
///     let v = x.0 * x.1 as f64 + y.0 * y.1 as f64;
///     let d = x.1 + y.1;
///     (v / 1.0_f64.max(d as f64), d)
/// };
/// let res = tc.each_root(empty, map, fold);
/// ```
/// ```ignore
/// // dp_v
/// let empty = 1;
/// let map = |&x: &u64, _: &()| (x + 1) % m;
/// let fold = |&x: &u64, &y: &u64| x * y % m;
/// let res = tc.each_root(empty, map, fold);
/// ```
/// ```ignore
/// // dp_p, abc036_d
/// let empty = (1, 1);
/// let map = |&x: &(u64, u64), _: &()| ((x.0 + x.1) % MOD, x.0);
/// let fold =
///     |&x: &(u64, u64), &y: &(u64, u64)| (x.0 * y.0 % MOD, x.1 * y.1 % MOD);
/// let res: Vec<_> = tc
///     .each_root(empty, map, fold)
///     .into_iter()
///     .map(|x| (x.0 + x.1) % MOD)
///     .collect();
/// assert!(res.iter().all(|&x| x == res[0]));
/// ```
/// ```ignore
/// // abc160_f
/// let mfb = ModFactorialBinom::new(n, MOD);
/// let f = |i| mfb.factorial(i);
/// let fr = |i| mfb.factorial_recip(i);
///
/// let empty = (0, 1, 1);
/// let map = |&x: &(usize, u64, u64), _: &()| {
///     (x.0 + 1, fr(x.0 + 1), x.1 * x.2 % MOD * f(x.0) % MOD)
/// };
/// let fold = |&x: &(usize, u64, u64), &y: &(usize, u64, u64)| {
///     (x.0 + y.0, (x.1 * y.1) % MOD, (x.2 * y.2) % MOD)
/// };
/// let res: Vec<_> = tc
///     .each_root(empty, map, fold)
///     .into_iter()
///     .map(|x| map(&x, &()).2)
///     .collect();
///     
/// // tdpc_tree
/// let res =
///     res.into_iter().fold(0_u64, |x, y| (x + y) % MOD) * mfb.recip(2) % MOD;
/// ```
///
/// # References
/// - <https://qiita.com/Kiri8128/items/a011c90d25911bdb3ed3>
///     - トポロジカルソートで求める話が書かれている。
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
