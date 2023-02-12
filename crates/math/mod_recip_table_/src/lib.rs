//! 素数 $m$ を法とした逆元のテーブル。

/// 素数 $m$ を法とした逆元のテーブル。
///
/// $\\gdef\\recip#1#2{#1\_{(#2)}\^{-1}}$
/// 以下、$i\^{-1}\\bmod j$ を $\\recip{i}{j}$ と書く。
///
/// 次で定められる \\(a = (a\_0, a\_1, \\dots, a\_n)\\) を返す。
/// \\[
/// a\_i = \\begin{cases}
/// \\recip{i}{m}, & \\text{if }\\recip{i}{m}\\text{ exists}; \\\\
/// 0, & \\text{otherwise}.
/// \\end{cases}
/// \\]
/// Note: $\\recip{i}{m}\\ne 0$。
///
/// # Idea
///
/// 次のことが成り立つ：
/// - $\\recip{0}{m}$ は存在しない。
/// - $\\recip{1}{m} = 1$ である。
///
/// $\\recip{(i+m)}{m} = \\recip{i}{m}$ なので、$i\\lt m$ について考える。
///
/// 各 $j$ ($1\\le j\\lt i$) については $\\recip{j}{m}$
/// が得られているとする。ただし、$m$ が素数なので、これらは常に存在する。
///
/// $m = q\\cdot i+r$ ($0\\le r\\lt i$) とおき、$\\recip{i}{m}$ を辺々掛けると
/// $$
/// \\begin{aligned}
/// m\\cdot\\recip{i}{m} &= q\\cdot i\\cdot\\recip{i}{m} + r\\cdot\\recip{i}{m} \\\\
/// 0 &\\equiv q + r\\cdot\\recip{i}{m} \\pmod{m}.
/// \\end{aligned}
/// $$
/// よって、
/// $$ \\recip{i}{m} \\equiv -q\\cdot\\recip{r}{m} \\pmod{m}. $$
/// $q\\gt 0$ と $\\recip{r}{m}\\gt 0$ であることから、
/// $$ \\recip{i}{m} = m - (q\\cdot\\recip{r}{m} \\bmod m) $$
/// となる。
///
/// $r\\lt i$ より $\\recip{r}{m}$
/// は既に得られているため、$\\recip{i}{m}$ が順次 $O(1)$ time で求まる。
///
/// # Notes
/// 一般のケースでは $\\gcd(r, m)=\\gcd(i, m)$ とは限らないので注意。
/// たとえば、$\\recip{3}{8}$ を求める際、$8 = 3\\cdot 2 + 2$ なので
/// $\\recip{2}{8}$ が必要になるが、これは存在しない。
///
/// 一般のケースで必要になる場合は、線形篩を用いる方法がある。See:
/// [`LinearSieve::recips`](../struct.LinearSieve.html#method.recips)
///
/// # Complexity
/// $O(n)$ time。
///
/// # Examples
/// ```
/// use nekolib::math::mod_recip_table_prime;
///
/// assert_eq!(mod_recip_table_prime(2, 3), [0, 1, 2]);
/// assert_eq!(mod_recip_table_prime(4, 5), [0, 1, 3, 2, 4]);
/// assert_eq!(mod_recip_table_prime(9, 5), [0, 1, 3, 2, 4].repeat(2));
/// ```
pub fn mod_recip_table_prime(n: u64, m: u64) -> Vec<u64> {
    let mut dp = vec![0; n as usize + 1];
    if 1 <= n {
        dp[1] = 1;
    }
    for i in 2..=n.min(m - 1) {
        let (q, r) = (m / i, m % i);
        if dp[r as usize] > 0 {
            dp[i as usize] = m - q * dp[r as usize] % m;
        }
    }
    for i in m..=n {
        dp[i as usize] = dp[(i - m) as usize];
    }
    dp
}

#[test]
fn test() {
    use factors_dup::FactorsDup;
    use gcd::Gcd;
    for m in (2_u64..=1000).filter(|m| m.factors_dup().count() == 1) {
        let n = m - 1;
        let actual = mod_recip_table_prime(n, m);
        for i in 0..=n {
            let recip = actual[i as usize];
            if recip == 0 {
                assert_ne!(i.gcd(m), 1);
            } else {
                assert!(recip < m);
                assert_eq!(i * recip % m, 1);
            }
        }
    }
}
