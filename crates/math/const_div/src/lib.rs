//! 定数除算。

use std::fmt::{self, Debug};

/// 定数除算。
///
/// 除算命令は重いので、加減算や乗算で置き換えることを考える。
/// 同じ値で何度も除算する際には、あらかじめ置き換える値を先に求めておくことで高速化できる。
///
/// Barrett reduction に基づく。$a\\lt n^2$ に対して、$\\lfloor a/n\\rfloor$ と $a\\bmod n$
/// を求めることができる。ちゃんと考察すれば、この制約は除けるはず。
/// 実際、コンパイラは同様の最適化を行う。
///
/// ```asm
/// example::div2:
///         mov     rax, rdi
///         shr     rax
///         ret
/// ```
/// ```asm
/// example::div3:
///         mov     rax, rdi
///         movabs  rcx, -6148914691236517205
///         mul     rcx
///         mov     rax, rdx
///         shr     rax
///         ret
/// ```
/// ```asm
/// example::div63:
///         movabs  rcx, 292805461487453201
///         mov     rax, rdi
///         mul     rcx
///         sub     rdi, rdx
///         shr     rdi
///         lea     rax, [rdi + rdx]
///         shr     rax, 5
///         ret
///
/// example::div64:
///         mov     rax, rdi
///         shr     rax, 6
///         ret
///
/// example::div65:
///         mov     rax, rdi
///         movabs  rcx, 1135184250689818561
///         mul     rcx
///         mov     rax, rdx
///         shr     rax, 2
///         ret
/// ```
///
/// ```
/// fn div63(rdi: u64) -> u64 {
///     let rdx = ((rdi as u128 * 0x410410410410411_u128) >> 64) as u64;
///     (((rdi - rdx) >> 1) + rdx) >> 5
/// }
///
/// fn div64(rdi: u64) -> u64 { rdi >> 6 }
///
/// fn div65(rdi: u64) -> u64 {
///     ((rdi as u128 * 0xFC0FC0FC0FC0FC1_u128) >> 66) as u64
/// }
///
/// for i in 0..=100000 {
///     assert_eq!(div63(i), i / 63);
///     assert_eq!(div64(i), i / 64);
///     assert_eq!(div65(i), i / 65);
/// }
/// ```
///
/// $$ \\begin{aligned}
/// \\lfloor n/63\\rfloor &= (((n-m)\\gg 1) + m)\\gg 5\\text{, where }
/// m=(n\\cdot\\lceil 2^{64}/63\\rceil)\\gg 64 \\\\
/// \\lfloor n/64\\rfloor &= n\\gg 6 \\\\
/// \\lfloor n/65\\rfloor &= (n\\cdot\\lceil 2^{66}/65\\rceil)\\gg 66
/// \\end{aligned} $$
///
/// 剰余算については、$n\\bmod d = n-\\lfloor n/d\\rfloor\\cdot d$ に基づく。
/// $d$ を掛ける際には定数乗算の最適化（加減算とシフトを用いるなど）を行っていそう。
///
/// 制約を除いた版：[`ConstDiv`]
///
/// [`ConstDiv`]: struct.ConstDiv.html
///
/// # Naming
/// 除数の 2 乗未満の入力を仮定することから `2` をつけている。
///
/// # References
/// - <https://rsk0315.hatenablog.com/entry/2021/01/18/065720#Barrett-reduction-%E3%81%AE%E8%A9%B1>
/// - <https://godbolt.org/z/snq4nvTP6>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstDiv2 {
    n: u64,
    recip: u128,
}

impl ConstDiv2 {
    pub fn new(n: u64) -> Self {
        let recip = 1_u64.wrapping_add(std::u64::MAX / n) as u128;
        Self { n, recip }
    }
    pub fn quot(&self, z: u64) -> u64 {
        if self.n == 1 {
            return z;
        }
        let x = ((self.recip * z as u128) >> 64) as u64;
        match x.checked_mul(self.n) {
            Some(xn) if xn <= z => x,
            _ => x - 1,
        }
    }
    pub fn rem(&self, z: u64) -> u64 {
        if self.n == 1 {
            return 0;
        }
        let x = ((self.recip * z as u128) >> 64) as u64;
        let v = z.wrapping_sub(x.wrapping_mul(self.n));
        if self.n <= v {
            v.wrapping_add(self.n)
        } else {
            v
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstDiv {
    n: u64,
    di: DivAlgo,
}

/// 除算のアルゴリズム。
///
/// 除数によってアルゴリズムを使い分ける。詳しくは [`ConstDiv`] のドキュメントを参照。
///
/// [`ConstDiv`]: struct.ConstDiv.html
#[derive(Clone, Copy, Eq, PartialEq)]
enum DivAlgo {
    Shr(u32),
    MulShr(u64, u32),
    MulAddShr(u64, u32),
    Ge(u64),
}
use DivAlgo::{Ge, MulAddShr, MulShr, Shr};

impl Debug for DivAlgo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match self {
            Shr(s) => format!("n >> {}", s),
            MulShr(m, s) => format!("(n * 0x{:016X}) >> {}", m, s),
            MulAddShr(m, s) => {
                format!("(n + ((n * 0x{:016X}) >> 64) >> 1) >> {}", m, s)
            }
            Ge(g) => {
                format!("if n >= 0x{:016x} {{ 1 }} else {{ 0 }}", g)
            }
        };
        f.write_str(res.as_str())
    }
}

impl ConstDiv {
    pub fn new(n: u64) -> Self {
        let ns = n.next_power_of_two().trailing_zeros();
        if n.is_power_of_two() {
            return Self { n, di: Shr(ns) };
        }
        if n.leading_zeros() == 0 {
            return Self { n, di: Ge(n) };
        }
        let nc = std::u64::MAX as u128;
        for p in 63 + ns..128 {
            let n_ = n as u128;
            let r = ((1_u128 << p) - 1) % n_;
            if (nc * (n_ - 1 - r)) >> p == 0 {
                let m = 1 + ((1_u128 << p) - 1 - r) / n_;
                return if m >> 64 == 0 {
                    Self { n, di: MulShr(m as u64, p) }
                } else {
                    Self { n, di: MulAddShr(m as u64, p - 1 - 64) }
                };
            }
        }
        unreachable!()
    }
    pub fn quot(&self, n: u64) -> u64 {
        match self.di {
            Shr(s) => n >> s,
            MulShr(m, s) => ((n as u128 * m as u128) >> s) as u64,
            MulAddShr(m, s) => {
                let tmp = ((n as u128 * m as u128) >> 64) as u64;
                (((n - tmp) >> 1) + tmp) >> s
            }
            Ge(g) if n >= g => 1,
            Ge(_) => 0,
        }
    }
    pub fn rem(&self, n: u64) -> u64 { n - self.quot(n) * self.n }
}

#[test]
fn test_small_2() {
    for n in 1..=500 {
        let cd = ConstDiv2::new(n);
        for a in 0..n * n {
            assert_eq!(cd.quot(a), a / n);
            assert_eq!(cd.rem(a), a % n);
        }
    }
}

#[test]
fn test_small() {
    for n in 1..=500 {
        let cd = ConstDiv::new(n);
        for a in 0..5 * n * n {
            assert_eq!(cd.quot(a), a / n);
            assert_eq!(cd.rem(a), a % n);
        }
        for a in 1..=5 * n * n {
            let a = std::u64::MAX - a;
            assert_eq!(cd.quot(a), a / n);
            assert_eq!(cd.rem(a), a % n);
        }
    }
}

#[test]
fn test_corner() {
    for &d in &[(1 << 63) - 1, 1 << 63, (1 << 63) + 1, std::u64::MAX] {
        let cd = ConstDiv::new(d);
        for &n in &[0, 1, d - 1, d, d.saturating_add(1), d.saturating_mul(2)] {
            assert_eq!(cd.quot(n), n / d);
            assert_eq!(cd.rem(n), n % d);
        }
    }
}
