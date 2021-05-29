//! 強連結成分分解。

/// lowlink に基づく強連結成分分解。
///
/// # Parameters
/// - `n`: 頂点数
/// - `vs`: 頂点集合
/// - `index`: 頂点から添字への番号づけをする関数
/// - `delta`: 頂点 `v` と関数 `search` を受け取る関数
///
/// `delta` は、`v` の各隣接頂点 `nv` に対して、`search(nv)`
/// を呼び出す必要がある。
///
/// # Return value
/// `index(v)` 番目の要素が `v` の属する強連結成分の番号である配列。
/// 番号づけはトポロジカル順に行われる。
///
/// # Examples
///
/// ```text
/// (0) ---> (1) ---> (3) ---> (5) ---> (6) ---> (7)
///  ^        |        |        ^        ^        |
///  |        v        v        |        |        |
/// (4) <--- (2)      (9)       +------ (8) <-----+
/// ```
///
/// ```
/// use nekolib::graph::scc;
///
/// let g = vec![
///     vec![1],
///     vec![2, 3],
///     vec![4],
///     vec![5, 9],
///     vec![0],
///     vec![6],
///     vec![7],
///     vec![8],
///     vec![5, 6],
///     vec![],
/// ];
/// let index = |&v: &usize| v;
/// let delta = |&v: &usize| g[v].iter().cloned();
/// let comp_id = scc(10, 0..10, index, delta);
///
/// assert_eq!(comp_id, vec![0, 0, 0, 1, 0, 3, 3, 3, 3, 2]);
/// ```
///
/// # Complexity
/// $O(|V|+|E|)$ 時間。
///
/// # References
/// - <https://niuez.github.io/posts/impl_abstract_dijkstra/>
pub fn scc<V, E>(
    n: usize,
    vs: impl Iterator<Item = V>,
    index: impl Fn(&V) -> usize + Copy,
    delta: impl Fn(&V) -> E + Copy,
) -> Vec<usize>
where
    E: Iterator<Item = V>,
{
    struct State {
        scc: Vec<Vec<usize>>,
        num: Vec<usize>,
        low: Vec<usize>,
        s: Vec<usize>,
        ins: Vec<bool>,
        t: usize,
    }

    fn dfs<V, E>(
        v: V,
        index: impl Fn(&V) -> usize + Copy,
        delta: impl Fn(&V) -> E + Copy,
        state: &mut State,
    ) where
        E: Iterator<Item = V>,
    {
        state.t += 1;
        let vi = index(&v);
        state.low[vi] = state.t;
        state.num[vi] = state.t;
        state.s.push(vi);
        state.ins[vi] = true;
        for nv in delta(&v) {
            let nvi = index(&nv);
            if state.num[nvi] == 0 {
                dfs(nv, index, delta, state);
                state.low[vi] = state.low[vi].min(state.low[nvi]);
            } else if state.ins[nvi] {
                state.low[vi] = state.low[vi].min(state.num[nvi]);
            }
        }
        if state.low[vi] == state.num[vi] {
            let mut tmp = vec![];
            loop {
                let nvi = state.s.pop().unwrap();
                state.ins[nvi] = false;
                tmp.push(nvi);
                if vi == nvi {
                    break;
                }
            }
            state.scc.push(tmp);
        }
    }

    let mut state = State {
        scc: vec![],
        num: vec![0; n],
        low: vec![0; n],
        s: vec![],
        ins: vec![false; n],
        t: 0,
    };

    for v in vs {
        if state.num[index(&v)] == 0 {
            dfs(v, index, delta, &mut state);
        }
    }

    let mut res = vec![0; n];
    for i in 0..state.scc.len() {
        for &c in &state.scc[i] {
            res[c] = state.scc.len() - i - 1;
        }
    }

    res
}
