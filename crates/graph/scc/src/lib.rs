//! 強連結成分分解。

/// lowlink に基づく強連結成分分解。
///
/// # Parameters
/// 後でちゃんと書きます。
///
/// [これ](https://niuez.github.io/posts/impl_abstract_dijkstra/)
/// をリスペクトしているつもり。
///
/// # Return value
/// `index(v)` 番目の要素が `v` の属する強連結成分の番号である配列。
/// 番号づけはトポロジカル順に行われる。
///
/// # Complexity
/// $O(|V|+|E|)$ 時間。
pub fn scc<G, V, I, D>(n: usize, vs: G, index: I, delta: D) -> Vec<usize>
where
    G: Iterator<Item = V>,
    V: Clone,
    I: Fn(&V) -> usize + Copy,
    D: Fn(&V, &mut dyn FnMut(V)) + Copy,
{
    struct State {
        scc: Vec<Vec<usize>>,
        num: Vec<usize>,
        low: Vec<usize>,
        s: Vec<usize>,
        ins: Vec<bool>,
        t: usize,
    }

    fn dfs<V, I, D>(v: V, index: I, delta: D, state: &mut State)
    where
        V: Clone,
        I: Fn(&V) -> usize + Copy,
        D: Fn(&V, &mut dyn FnMut(V)) + Copy,
    {
        state.t += 1;
        let vi = index(&v);
        state.low[vi] = state.t;
        state.num[vi] = state.t;
        state.s.push(vi);
        state.ins[vi] = true;
        delta(&v, &mut |nv| {
            let nvi = index(&nv);
            if state.num[nvi] == 0 {
                dfs(nv, index, delta, state);
                state.low[vi] = state.low[vi].min(state.low[nvi]);
            } else if state.ins[nvi] {
                state.low[vi] = state.low[vi].min(state.num[nvi]);
            }
        });
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
