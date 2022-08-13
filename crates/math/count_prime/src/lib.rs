//! 素数の数え上げ。

/// 素数の数え上げ。
///
/// $n$ 以下の素数の個数 $\\pi(n)$ を返す。
///
/// # Idea
/// 次の二つのパートで構成される。
///
/// 1. Eratosthenes の篩を行う際に篩われる個数を小さい素数について数える
/// 2. 大きい素数の積で表される合成数を数える
///
/// $p$ 以下まで篩った際に残っている $2$ 以上 $v$ 以下の整数の個数を $S(v, p)$ と表す。
/// $p$ が合成数または $p\^2 \\gt n$ のときは変化しないので、$S(v, p) = S(v, p-1)$ が成り立つ。
/// そうでないとき、すなわち $p$ が $v\^2$ 未満の素数であれば次の式が成り立つ。
/// $$ S(v, p) = S(v, p-1) - (S(\\lfloor v/p\\rfloor, p-1) - S(p-1, p-1)). $$
/// このことから、$S(\\lfloor n/\\bullet\\rfloor, \\bullet)$ の値を管理すればよく、
/// $O(\\sqrt{n})$ space の DP を構成できる。この DP を $p\\le \\sqrt\[4\]{n}$ に対して行う。
///
/// $\\sqrt\[4\]{n}$ 以下の素数について篩った後に残っている整数は、$\\sqrt\[4\]{n}$
/// より大きい素因数たかだか $3$ つの積で表されることに注意する。
/// これを利用して残っている合成数を数えるが、これは
/// $S(\\lfloor n/\\bullet\\rfloor, \\lfloor\\sqrt\[4\]{n}\\rfloor)$ を用いて計算できる。
///
/// # Complexity
/// $O(\\sqrt{n})$ space, $O(n^{3/4} / \\log(n))$ time.
///
/// # Examples
/// ```
/// use nekolib::math::prime_pi;
///
/// assert_eq!(prime_pi(10), 4);
/// assert_eq!(prime_pi(100), 25);
/// assert_eq!(prime_pi(1000), 168);
/// assert_eq!(prime_pi(10000), 1229);
/// assert_eq!(prime_pi(100_000_000_000), 4118054813);
/// ```
///
/// # References
/// - <https://rsk0315.hatenablog.com/entry/2021/05/18/015511>
/// - <https://judge.yosupo.jp/submission/7976>
/// - <https://math314.hateblo.jp/entry/2016/06/05/004332>
/// - <https://projecteuler.net/thread=10;page=5#111677>
#[deprecated]
fn prime_pi(n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }

    let v = floor_sqrt(n);
    let mut s = (v + 1) / 2;
    let mut smalls: Vec<_> = (0..s).collect();
    let mut roughs: Vec<_> = (0..s).map(|i| 2 * i + 1).collect();
    let mut larges: Vec<_> =
        (0..s).map(|i| (n / (2 * i + 1) - 1) / 2).collect();
    let mut skip = vec![false; v + 1];
    let half = |i: usize| (i - 1) / 2;
    let mut pc = 0;
    for p in (3..=v).step_by(2) {
        if skip[p] {
            continue;
        }
        let q = p * p;
        if q * q > n {
            break;
        }
        skip[p] = true;
        for i in (q..=v).step_by(2 * p) {
            skip[i] = true;
        }
        let mut ns = 0;
        for k in 0..s {
            let i = roughs[k];
            if skip[i] {
                continue;
            }
            let d = i * p;
            larges[ns] = larges[k] + pc
                - if d <= v {
                    larges[smalls[d / 2] - pc]
                } else {
                    smalls[half(n / d)]
                };
            roughs[ns] = i;
            ns += 1;
        }
        s = ns;
        let mut i = half(v);
        for j in (p..=((v / p) - 1) | 1).rev().step_by(2) {
            let c = smalls[j / 2] - pc;
            let e = (j * p) / 2;
            while i >= e {
                smalls[i] -= c;
                i -= 1;
            }
        }
        pc += 1;
    }
    larges[0] +=
        s.wrapping_add(2_usize.wrapping_mul(pc.wrapping_sub(1))) * (s - 1) / 2;
    for k in 1..s {
        larges[0] -= larges[k];
    }
    for l in 1..s {
        let q = roughs[l];
        let m = n / q;
        let e = smalls[half(m / q)] - pc;
        if e <= l {
            break;
        }
        let mut t = 0;
        for k in l + 1..=e {
            t += smalls[half(m / roughs[k])];
        }
        larges[0] += t - (e - l) * (pc + l - 1);
    }
    larges[0] + 1
}

fn floor_sqrt(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    let mut lo = 1;
    let mut hi = n;
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        match mid.overflowing_mul(mid) {
            (mid2, false) if mid2 <= n => lo = mid,
            _ => hi = mid,
        }
    }
    lo
}
