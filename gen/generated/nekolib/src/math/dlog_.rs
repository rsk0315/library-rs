//! 離散対数。

use super::const_div;
use super::divisors_;
use super::factors_;
use super::gcd_recip_;
use super::mod_pow_;
use super::totient_phi_;

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
/// $b\^z \\equiv a \\pmod{n}$ なる $z\\ge 0$ が存在すれば、そのうち最小のものを返す。
///
/// # Idea
/// `todo!()`
///
/// # Complexity
/// $\\tilde{O}(\\sqrt{n})$ time.
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

    let b = b % n;
    let a = a % n;

    let nf: BTreeMap<_, _> = factors(n).collect();
    let tail = factors(b)
        .map(|(p, e)| (*nf.get(&p).unwrap_or(&0) + e - 1) / e)
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

    bsgs(bb, b, a, n, cd, c).map(|head| tail + head)
}

fn bsgs(bb: u64, b: u64, a: u64, n: u64, cd: ConstDiv, c: u64) -> Option<u64> {
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
    for i in 0..=n / step {
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
