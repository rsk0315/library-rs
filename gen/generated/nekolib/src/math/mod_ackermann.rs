//! Ackermann 関数。

use super::mod_pow;
use super::mod_tetration;

use mod_pow::ModPow;
use mod_tetration::ModTetration;

/// Ackermann 関数。
///
/// 与えられた $(a, b, n)$ に対して $A(a, b)\\bmod n$ を返す。
/// 自然数 $a$, $b$ に対して Péter--Ackermann 関数 $A(a, b)$ は次のように定義される。
/// - $A(0, b) = b+1$,
/// - $A(a+1,0) = A(a, 1)$,
/// - $A(a+1, b+1) = A(a, A(a+1, b))$.
///
/// $\\gdef{\\hyper}{\\operatorname{hyper}}$
/// 以下の性質が知られている。
/// $$ a\\gt 0 \\implies \\hyper\_a(2, b+3)-3. $$
/// ただし、$\\hyper\_a$ は $a$ 番目のハイパー演算子である。
///
/// # Idea
/// $a\\le 4$ までは [`mod_pow`] や [`mod_tetration`] を用いて簡単に求められる。
/// [`mod_tetration`] と同様に、指数部が十分大きくなると値が一定になるので、それを利用する。
///
/// $A(5, b) = \\hyper\_5(2, b+3)-3 = \\underbrace{{}^{{}^{{}^2\\cdots} 2} 2}\_{b+3\\text{ many}} - 3$
/// となるが、$A(5, 0)$ は現実的な値として求められる。
/// $$ \\begin{aligned}
/// A(5, 0) &= {}^{{}^2 2} 2 - 3 \\\\
/// &= {}^4 2 - 3 \\\\
/// &= 2^{2^{2^2}} - 3 \\\\
/// &= 2^{2^4} - 3 \\\\
/// &= 2^{16}-3 = 65533.
/// \\end{aligned} $$
///
/// 一方で、$A(5, 1)$ は次のようになる。
/// $$ \\begin{aligned}
/// A(5, 1) &= {}^{{}^{{}^2 2} 2} 2 - 3 \\\\
/// &= {}^{65536} 2 - 3 \\\\
/// &= \\underbrace{2^{2^{\\cdots^2}}}\_{65536\\text{ many}}-3.
/// \\end{aligned} $$
///
/// ここで [`mod_tetration`] の議論を思い出すと、${}^b a \\bmod{n}$ は $b\\ge 2\\log(n)$
/// であれば一定値となる。引数の型が [`u64`] である今、$b$ として意味があるのは高々
/// $2\\log(2^{64}) = 128$ 程度であり、$A(5, 1)$ はそれを十分に上回る。
/// すなわち、$A(5, 1)\\bmod n = A(4, 2\\log(n))\\bmod n$ として計算してしまってよい[^bd]。
///
/// さらに、$A(6, 0) = A(5, 1)$ などから、$a\\ge 6$ についても同様にでき、次のようにできる。
/// - $A(0, b) \\equiv b+1 \\pmod{n}$,
/// - $A(1, b) \\equiv b + 2 \\pmod{n}$,
/// - $A(2, b) \\equiv 2b + 3 \\pmod{n}$,
/// - $A(3, b) \\equiv 2^{b+3} - 3 \\pmod{n}$,
/// - $A(4, b) \\equiv {}^{b+3} 2 - 3 \\pmod{n}$,
/// - $A(5, 0) \\equiv 65533 \\pmod{n}$,
/// - $A(5, b) \\equiv A(4, 2\\log(n)) \\pmod{n}$ for $b\\ge 1$,
/// - $A(a, b) \\equiv A(4, 2\\log(n)) \\pmod{n}$ for $a\\ge 6$.
///
/// [^bd]: 32768-bit 整数を受け取るような状況ではよくなさそう。512-word
/// の多倍長整数と思うとそこそこ現実的な気もする？ だとしても $A(5, 2)$ はもう現実的じゃなさそう。
///
/// [`mod_pow`]: fn.mod_pow.html
/// [`mod_tetration`]: fn.mod_tetration.html
///
/// # Complexity
/// |入力|時間計算量|
/// |---|---|
/// |$a\\le 2$ or $(a, b) = (5, 0)$|$O(1)$|
/// |$a=3$|$O(\\log(b))$|
/// |otherwise|$O(\\sqrt{n})$|
///
/// # Examples
/// ```
/// use nekolib::math::ModAckermann;
///
/// let n = 10_u64.pow(9);
/// assert_eq!(2_u64.mod_ackermann(5, n), 13);
/// assert_eq!(3_u64.mod_ackermann(7, n), 1_021);
/// assert_eq!(4_u64.mod_ackermann(2, n), 719_156_733);
/// assert_eq!(4_u64.mod_ackermann(3, n), 437_428_733);
/// assert_eq!(4_u64.mod_ackermann(8, n), 432_948_733);
/// assert_eq!(9_u64.mod_ackermann(9, n), 432_948_733);
/// ```
pub trait ModAckermann {
    fn mod_ackermann(self, b: Self, n: Self) -> Self;
}

macro_rules! impl_uint {
    ($t:ty) => {
        impl ModAckermann for $t {
            fn mod_ackermann(self, b: Self, n: Self) -> Self {
                let sub_3 = |z| match n {
                    1 => 0,
                    2 => 1 - z,
                    3 => z,
                    _ if z >= 3 => z - 3,
                    _ => z + n - 3,
                };
                match (self, b) {
                    (0, _) => (b + 1) % n,
                    (1, _) => (b + 2) % n,
                    (2, _) => (2 * b + 3) % n,
                    (3, _) => sub_3((2 as $t).mod_pow(b + 3, n)),
                    (4, _) => sub_3((2 as $t).mod_tetration(b + 3, n)),
                    (5, 0) => (65533_u128 % n as u128) as $t, // for u8
                    _ => sub_3((2 as $t).mod_tetration(n, n)),
                }
            }
        }
    };
    ( $($t:ty)* ) => { $(impl_uint!($t);)* };
}

impl_uint!(u8 u16 u32 u64 u128 usize);

#[test]
fn test() {
    let n = 14_u64.pow(8);
    let res: u64 = (0..=7).map(|a| a.mod_ackermann(a, n)).sum();
    assert_eq!(res % n, 452_774_460);
}
