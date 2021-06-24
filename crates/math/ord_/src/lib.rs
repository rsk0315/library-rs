//! 位数。

use factors_::factors_dup;
use mod_pow_::mod_pow;
use totient_phi_::totient_phi;

/// 位数。
///
/// $a^z \\equiv 1 \\pmod{n}$ なる $z\\ge 0$ が存在すれば、そのうち最小のものを返す。
///
/// # Complexity
/// $\\phi(n)$ に対する素因数列挙にかかる時間に加え、各素因数に対して
/// $O(\\log(\\phi(n)))$ 時間。試し割り法では $O(\\sqrt{n})$ 時間。
///
/// # Examples
/// ```
/// use nekolib::math::ord;
///
/// assert_eq!(ord(7, 10), Some(4));
/// assert_eq!(ord(1, 3), Some(1));
/// assert_eq!(ord(22, 30), None);
/// ```
///
/// # Suggestions
/// [`dlog`] と同様、$\\phi$ 関数と素因数列挙に関して引数で渡したいかも。
///
/// [`dlog`]: fn.dlog.html
pub fn ord(a: u64, n: u64) -> Option<u64> {
    let mut q = totient_phi(n);
    for e in factors_dup(q) {
        if mod_pow(a, q / e, n) == 1 {
            q /= e;
        }
    }
    if mod_pow(a, q, n) == 1 {
        Some(q)
    } else {
        None
    }
}
