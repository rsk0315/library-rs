//! 定数除算。

use std::fmt::Debug;

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
/// 剰余算についても調べる。`todo!()`
///
/// # References
/// - <https://rsk0315.hatenablog.com/entry/2021/01/18/065720#Barrett-reduction-%E3%81%AE%E8%A9%B1>
/// - <https://godbolt.org/z/snq4nvTP6>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstDiv {
    n: u64,
    recip: u128,
}

impl ConstDiv {
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

#[test]
fn test() {
    for n in 1..=1000 {
        let cd = ConstDiv::new(n);
        for a in 0..n * n {
            assert_eq!(cd.quot(a), a / n);
            assert_eq!(cd.rem(a), a % n);
        }
    }
}
