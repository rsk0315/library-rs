//! 周期検出。

/// 周期検出を行う。
///
/// 与えられた $x\_0$ と $f$ を用いて $x\_i = f(x\_{i-1})$ ($i > 0$) として
/// 定義される列 $\\{x\_i\\}\_{i=0}\^\\infty$ の周期検出を行う。
/// $x\_{\\mu} = x\_{\\mu+\\lambda}$ なる最小の $(\\mu, \\lambda)$ を返す。
///
/// # Requirements
/// $f$ は参照透過である。
///
/// # Notes
/// 共通の $f$ に対して、異なる複数の $x\_0$ から周期検出を行いたい場合は、
/// この関数を複数回呼ぶよりも高速なアルゴリズムがあると思われる。
/// ある $x\_0$ での出力が $(\\mu, \\lambda)$ であれば、
/// $x\_i$ ($1\\le i\< \\mu$) での出力は $(\\mu-i, \\lambda)$、
/// $x\_i$ ($\\mu\\le i<\\mu+\\lambda$) での出力は $(0, \\lambda)$ とわかる。
/// さらに、これら $\\mu+\\lambda$ 個以外の $x\'$ についても、
/// $f\^i(x\')$ がこれらのいずれかと等しくなる $i$ が存在すれば、
/// $\\Theta(\\mu)$ 回の $f$ の呼び出しで $x\', f(x\'), \\dots, f\^{i-1}(x\')$ の出力もわかるはず。
///
/// # Complexity
/// $\\Theta(\\mu+\\lambda)$ 回の $f$ の呼び出しを行う。
///
/// # Examples
/// ```
/// use nekolib::algo::cycle_mu_lambda;
///
/// // 3, 9, 11, 9, 11, ...
/// assert_eq!(cycle_mu_lambda(3, |x| x * x % 14), (1, 2));
/// // 2, 6, 4, 5, 1, 3, 2, ...
/// assert_eq!(cycle_mu_lambda(2, |x| x * 3 % 7), (0, 6));
/// ```
///
/// ```
/// use nekolib::algo::cycle_mu_lambda;
///
/// assert_eq!(cycle_mu_lambda(0, |x| x), (0, 1));
/// ```
pub fn cycle_mu_lambda<T, F>(x0: T, f: F) -> (usize, usize)
where
    T: Eq + Clone,
    F: Fn(T) -> T,
{
    let mut tor = f(x0.clone());
    let mut har = f(tor.clone());

    while tor != har {
        tor = f(tor);
        har = f(f(har));
    }

    let mut tor = x0;
    let mut mu = 0;
    while tor != har {
        tor = f(tor);
        har = f(har);
        mu += 1;
    }

    let mut lambda = 1;
    har = f(tor.clone());
    while tor != har {
        har = f(har);
        lambda += 1;
    }

    (mu, lambda)
}

/// $n$ 項目を求める。
///
/// 与えられた $x\_0$ と $f$ を用いて $x\_i = f(x\_{i-1})$ ($i > 0$) として
/// 定義される列 $\\{x\_i\\}\_{i=0}\^\\infty$ の $n$ 項目を求める。
///
/// # Requirements
/// $f$ は参照透過である。
///
/// # Complexuty
/// $x\_{\\mu} = x\_{\\mu+\\lambda}$ なる最小の $(\\mu, \\lambda)$
/// に対し、$O(\\min\\{n, \\mu+\\lambda\\})$ time.
///
/// # Examples
/// ```
/// use nekolib::algo::cycle_nth;
///
/// let x0 = 0;
/// let f = |x| (x + 1) % 100; // (mu, lambda) = (0, 100)
/// assert_eq!(cycle_nth(x0, f, 0), 0);
/// assert_eq!(cycle_nth(x0, f, 99), 99);
/// assert_eq!(cycle_nth(x0, f, 100), 0);
/// assert_eq!(cycle_nth(x0, f, 1000), 0);
/// assert_eq!(cycle_nth(x0, f, 10_usize.pow(9)), 0);
///
/// let x0 = -10;
/// let f = |x| (x + 1) % 100; // (mu, lambda) = (10, 100)
/// assert_eq!(cycle_nth(x0, f, 0), -10);
/// assert_eq!(cycle_nth(x0, f, 99), 89);
/// assert_eq!(cycle_nth(x0, f, 100), 90);
/// assert_eq!(cycle_nth(x0, f, 1000), 90);
/// assert_eq!(cycle_nth(x0, f, 10_usize.pow(9)), 90);
/// ```
pub fn cycle_nth<T, F>(x0: T, f: F, n: usize) -> T
where
    T: Eq + Clone,
    F: Fn(T) -> T,
{
    if n == 0 {
        return x0;
    }
    if n == 1 {
        return f(x0);
    }

    let mut tor = f(x0.clone());
    let mut har = f(tor.clone());
    let mut i = 2;
    while i + 2 <= n && tor != har {
        tor = f(tor);
        har = f(f(har));
        i += 2;
    }
    if i == n {
        return har;
    }
    if i + 1 == n {
        return f(har);
    }

    let mut tor = x0.clone();
    let mut mu = 0;
    while tor != har {
        tor = f(tor);
        har = f(har);
        mu += 1;
    }

    let mut lambda = 1;
    har = f(tor.clone());
    while tor != har {
        har = f(har);
        lambda += 1;
    }

    let n = if n <= mu { n } else { mu + (n - mu) % lambda };
    let mut x = x0;
    for _ in 0..n {
        x = f(x);
    }
    x
}
