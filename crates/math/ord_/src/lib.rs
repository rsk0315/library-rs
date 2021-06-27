//! 位数。

use carmichael_lambda_::carmichael_lambda;
use const_div::ConstDiv;
use factors_::factors_dup;
use mod_pow_::mod_pow_with_cd;

/// 位数。
///
/// ~~$a^z \\equiv 1 \\pmod{n}$ なる $z\\gt 0$ が存在すれば、そのうち最小のものを返す。~~
///
/// $0\\le a\\lt n$ とする。
///
/// 嘘じゃん。
///
// # Complexity
// $\\lambda(n)$ に対する素因数列挙にかかる時間に加え、各素因数に対して
// $O(\\log(\\lambda(n)))$ 時間。試し割り法では $O(\\sqrt{n})$ 時間。
//
// # Examples
// ```
// use nekolib::math::ord;
//
// assert_eq!(ord(7, 10), Some(4));
// assert_eq!(ord(1, 3), Some(1));
// assert_eq!(ord(22, 30), None);
// ```
//
// # Suggestions
// [`dlog`] と同様、$\\lambda$ 関数と素因数列挙に関して引数で渡したいかも。
//
// [`dlog`]: fn.dlog.html
pub fn ord(a: u64, n: u64) -> Option<u64> {
    if n == 0 {
        panic!("modulo must be positive")
    }
    match (a, n) {
        (0, 1) => return None,
        (0, _) => return Some(0),
        (1, _) => return Some(0),
        _ => {}
    }

    let mut q = carmichael_lambda(n);
    let cd = ConstDiv::new(n);
    for e in factors_dup(q) {
        if mod_pow_with_cd(a, q / e, cd) == 1 {
            q /= e;
        }
    }
    if mod_pow_with_cd(a, q, cd) == 1 {
        Some(q)
    } else {
        None
    }
}

#[test]
fn test() {
    let n_max = 200;

    for n in 2..=n_max {
        for a in 0..n {
            let actual = ord(a, n);
            let mut x = 1;
            let expected = (0..n).find_map(|i| {
                x = x * a % n;
                if x == 1 {
                    Some(i)
                } else {
                    None
                }
            });
            // assert_eq!(actual, expected);
        }
    }
}
