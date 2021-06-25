//! 離散対数。

use std::collections::BTreeMap;

use const_div::ConstDiv;
use divisors_::divisors;
use factors_::factors;
use gcd_recip_::gcd_recip;
use mod_pow_::mod_pow_with_cd;
use totient_phi_::totient_phi;

/// 離散対数。
///
/// $n$ を法とする $\\mathrm{dlog}\_b(a)$。
/// $b^z \\equiv a \\pmod{n}$ なる $z\\ge 0$ が存在すれば、そのうち最小のものを返す。
///
/// # Idea
/// $1\\to b\\to b^2\\to\\dots\\pmod{n}$ でできる functional graph を考えると、$\\rho$
/// の字状になっていることに気づく。しっぽの長さ $\\mu$ は高々 $\\log(n)$、頭の長さ
/// $\\lambda$ は $\\phi(n)$ の約数になっていることが示せる。
///
/// そこで、まずしっぽの部分に解があるかを愚直に $O(\\mu)$ 時間で調べる。
/// 見つからなければ、頭の部分に解があるかを $o(\\lambda)$ 時間で調べる。
/// $O(\\sqrt{\\lambda}\\log(\\lambda))$ 時間のアルゴリズムを紹介する。
///
/// $\\rho$ の頭の部分は $b^\\mu\\to b^{\\mu+1}\\to\\dots\\to b^{\\mu+\\lambda}\\equiv b^\\mu$
/// である。このとき、${}^\\forall i\\ge\\mu$ について
/// $b^{i+1}\\cdot b^{\\lambda-1} \\equiv b^i \\pmod{n}$ が成り立つ。
/// すなわち、$b^{\\lambda-1}$ を掛けることで、$\\to$ をひとつ戻ることができる[^1]。
///
/// [^1]: $\\mu\\gt 0$ なら、 $b^\\mu$ は $b^{\\mu-1}$ と $b^{\\mu+\\lambda-1}$
/// ($\\neq b^{\\mu-1}$) のふたつから $\\to$ が入ってくるが、$\\rho$
/// の頭に含まれる後者に戻る。
///
/// そこで、$z = \\mu+i\\cdot\\lfloor\\sqrt{\\lambda}\\rfloor+j$ ($0\\le j\\lt\\sqrt{\\lambda}$)
/// とし、$(i, j)$ を探すことを考える。
/// $$ \\begin{aligned}
/// b^{\\mu+i\\cdot\\lfloor\\sqrt{\\lambda}\\rfloor+j} &\\equiv a \\\\
/// b^{\\mu+i\\cdot\\lfloor\\sqrt{\\lambda}\\rfloor+j}\\cdot (b^{\\lambda-1})^j
///  &\\equiv a\\cdot (b^{\\lambda-1})^j \\\\
/// b^{\\mu+i\\cdot\\lfloor\\sqrt{\\lambda}\\rfloor}
///  &\\equiv a\\cdot (b^{\\lambda-1})^j \\\\
/// \\end{aligned} $$
/// これより、$i$ と $j$ を分離でき、$a\\cdot(b^{\\lambda-1})^j\\mapsto j$
/// の連想配列を作っておくことで、$b^{\\mu+i\\cdot\\lfloor\\sqrt{\\lambda}\\rfloor}$
/// に対応する要素があれば返せる。ただし、複数の $j$ について
/// $a\\cdot(b^{\\lambda-1})^j$ が同じ値を取りうるので注意する必要がある。
///
/// $j$ による小さい幅で連想配列を作り、$i$ による大きい幅でそれにアクセスする様子から
/// baby-step giant-step algorithm と呼ばれている。
///
/// 以下では、$\\mu$ と $\\lambda$ を求める方法について述べる。`todo!()`
///
/// # Complexity
/// $O(\\sqrt{n} + \\sqrt{\\lambda}\\log(\\lambda))$ time.
/// ここで、$\\lambda$ は最悪 $\\Theta(n)$ である。
///
/// `todo!()`
///
/// # Examples
/// ```
/// use nekolib::math::dlog;
///
/// assert_eq!(dlog(6, 5, 13), Some(9));
/// assert_eq!(dlog(27, 3, 30), Some(3));
/// assert_eq!(dlog(2, 0, 4), Some(2));
/// assert_eq!(dlog(0, 1, 2), Some(0));
/// assert_eq!(dlog(2, 3, 10), None);
/// ```
///
/// # References
/// - <https://divinejk.hatenablog.com/entry/2021/02/07/133155>
///
/// # Suggestions
/// 内部で Euler の $\\phi$ 関数と約数列挙と素因数分解が必要となるので、
/// 篩などを用いて高速化が図れる場合に対応しやすいようにしてみるとよい？
///
/// ```ignore
/// pub fn dlog<I, J>(
///     b: u64, a: u64, n: u64,
///     euler_phi: impl Fn(u64) -> u64,
///     divisors: impl Fn(u64) -> I,
///     factors: impl Fn(u64) -> J,
/// ) -> Option<u64>
/// where
///     I: Iterator<Item = u64>,
///     J: Iterator<Item = (u64, u32)>,
/// { ... }
/// ```
///
/// 一方で、そうでない場合にわざわざ関数を渡す必要があるので単に面倒かも。
/// お手軽パターンと二つ用意するといいかも？（デフォルト引数欲しいね）
///
/// とはいえ、結局 BS/GS パートが律速で $O(\\sqrt{n\\log(n)})$ 時間とかになってしまいそう。
pub fn dlog(b: u64, a: u64, n: u64) -> Option<u64> {
    match (b, a, n) {
        (_, _, 0) => panic!("modulo must be positive"),
        (_, _, 1) => return Some(0),
        (_, 1, _) => return Some(0),
        (0, 0, _) => return Some(1),
        (0, _, _) => return None,
        (1, _, _) => return None,
        _ => {}
    }

    let tail = factors(b)
        .map(|(p, e)| {
            let mut f = 0;
            let mut n = n;
            while n % p == 0 {
                n /= p;
                f += 1;
            }
            (f + e - 1) / e
        })
        .max()
        .unwrap() as u64;

    let cd = ConstDiv::new(n);
    let mut bpow = 1;
    for i in 0..tail {
        if bpow == a {
            return Some(i);
        }
        bpow = cd.rem(bpow * b);
    }

    let bb = bpow;
    if a == 0 {
        return if bb == 0 { Some(tail) } else { None };
    }
    if a % gcd_recip(bb, n).0 != 0 {
        return None;
    }

    let c = divisors(totient_phi(n))
        .find(|&c| cd.rem(bb * mod_pow_with_cd(b, c, cd)) == bb)
        .unwrap();

    bsgs(bb, b, a, cd, c).map(|head| tail + head)
}

fn bsgs(bb: u64, b: u64, a: u64, cd: ConstDiv, c: u64) -> Option<u64> {
    let step = (1..).find(|&i| i * i >= c).unwrap();
    let seen = {
        let mut seen = BTreeMap::new();
        let baby_recip = mod_pow_with_cd(b, c - 1, cd);
        let mut x = a;
        for i in 0..step {
            seen.entry(x).or_insert(i);
            x = cd.rem(x * baby_recip);
        }
        seen
    };
    let giant = mod_pow_with_cd(b, step, cd);
    let mut x = bb;
    for i in 0..=c / step {
        if let Some(&e) = seen.get(&x) {
            return Some(i * step + e);
        }
        x = cd.rem(x * giant);
    }
    None
}

#[test]
fn test() {
    use std::collections::btree_map::Entry::{Occupied, Vacant};
    let n_max = 200;

    for n in 1..=n_max {
        for b in 0..n {
            let expected = {
                let mut expected = BTreeMap::new();
                let mut x = 1 % n;
                for i in 0.. {
                    match expected.entry(x) {
                        Vacant(v) => v.insert(i),
                        Occupied(_) => break,
                    };
                    x = x * b % n;
                }
                expected
            };

            for a in 0..n {
                let z = dlog(b, a, n);
                assert_eq!(z, expected.get(&a).cloned());
            }
        }
    }
}
