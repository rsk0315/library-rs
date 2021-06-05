//! 並列二分探索。

use stateful_predicate::StatefulPred;

/// 並列二分探索を行う。
///
/// 状態によって返り値の異なる述語を考える。
/// 各クエリに対して、初めて偽になる状態の番号を返す。
/// 常に真となる場合、状態の個数を返す。
///
/// # Requirements
/// 状態 $j$ の述語に $x\_i$ を与えたときの返り値を $f\_j(x\_i)$ とする。
/// $f\_j(x\_i)$ が偽となるとき、${}\^\\forall j\' > j$ について $f\_{j\'}(x\_i)$ も偽となる。
///
/// 直感的には、状態が進むにつれて真となる条件が厳しくなる述語を指す。
///
/// # Idea
/// $i$ 番目のクエリについて、区間 $[\\mathrm{ok}\_i, \\mathrm{bad}\_i)$ を管理する。
/// これは、$f\_{\\mathrm{ok}\_i}(x\_i)$ は真、$f\_{\\mathrm{bad}\_i}(x\_i)$
/// は偽になることを意味する。
/// 状態の個数を $m$ として、初期値は $[-1, m)$ とする。
///
/// 状態を進めていきながら、ある $i$ に対して
/// 状態 $j = \\lfloor(\\mathrm{ok}\_i+\\mathrm{bad}\_i)/2\\rfloor$ となったとき、
/// $f\_j(x\_i)$ を計算する。これにより、答えの範囲が半分に絞れる。
/// この一連の計算を $\\log\_2(m)+O(1)$ 回繰り返せばよい。
///
/// 各クエリについて独立に計算するのではなく、
/// 一つの述語を共有して並列に処理することで、計算量を削減できる。
///
/// 毎ループで状態 $m-1$ まで遷移する必要はなく、
/// $f\_j(x\_i)$ を計算したい $i$ が存在する最大の $j$ まで見ればよい。
///
/// # Notes
/// 永続データ構造が作れるのであれば、単にそれを用いて各クエリについて二分探索を行えばよい。
/// また、クエリの個数が少なく、述語の計算コストが高くない場合は、
/// 各々について線形探索を行う方が高速な場合もありうる。
///
/// # Complexity
/// 状態 $0$ から状態 $m-1$ までの遷移を高々 $\\log\_2(m)+O(1)$ 回行う。
/// また、各クエリに対して述語の呼び出しを $\\log\_2(m)+O(1)$ 回行う。
///
/// # Examples
/// ```
/// use nekolib::algo::parallel_bisect;
/// use nekolib::traits::StatefulPred;
///
/// struct Neko(i32);
/// impl Neko {
///     pub fn new() -> Self { Self(0) }
/// }
///
/// /// 状態 `i` において値 `10 * i` を持ち、値 `100` を最終状態とする。
/// /// この値より大きい値に対して真を返す。
/// impl StatefulPred for Neko {
///     type Input = i32;
///     fn count(&self) -> usize { 11 }
///     fn next(&mut self) {
///         if self.0 < 100 { self.0 += 10; }
///     }
///     fn pred(&self, &x: &i32) -> bool { x > self.0 }
///     fn reset(&mut self) { self.0 = 0; }
/// }
///
/// let qs = vec![0, 1, 32, 60, 89, 99, 100, 101, 500];
/// assert_eq!(
///     parallel_bisect(Neko::new(), qs),
///     vec![0, 1, 4, 6, 9, 10, 10, 11, 11]
/// );
/// ```
pub fn parallel_bisect<S: StatefulPred>(
    mut s: S,
    q: Vec<S::Input>,
) -> Vec<usize> {
    let sn = s.count();
    let qn = q.len();
    let mut ok = vec![0; qn];
    let mut bad = vec![sn + 1; qn];

    loop {
        let mut ev = vec![vec![]; sn + 1];
        let mut max = None;
        for i in 0..qn {
            if bad[i] - ok[i] <= 1 {
                continue;
            }
            let mid = ok[i] + (bad[i] - ok[i]) / 2;
            ev[mid].push(i);
            max = Some(max.unwrap_or(0).max(mid));
        }
        if max.is_none() {
            break;
        }

        s.reset();
        for j in 1..=max.unwrap() {
            for &i in &ev[j] {
                if s.pred(&q[i]) {
                    ok[i] = j;
                } else {
                    bad[i] = j;
                }
            }
            s.next();
        }
    }

    bad.into_iter().map(|x| x - 1).collect()
}
