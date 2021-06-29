//! tetration。

use const_div::ConstDiv;
use euler_phi_::euler_phi;

/// tetration。
///
/// ${}^b a\\bmod n$ を求める。${}^\\bullet \\bullet$ は次のように定義される。
/// $$ {}^b a = \\begin{cases}
/// 1, & \\text{if } b = 0; \\\\
/// a^{\\left({}^{(b-1)} a\\right)}, & \\text{if } b \\ge 1.
/// \\end{cases} $$
/// 直感的に書けば、$\\underbrace{a^{(a^{(\\cdots ^a)})}}\_{b\\text{ many}} \\bmod n$ である。
///
/// # Idea
/// 大変大きくなりうる $z$ に対して $a^z\\bmod n$ を求めることを考える。
/// このとき、[`dlog`] の [Idea] と同じ議論から、ある $\\mu$, $\\lambda$ が存在して
/// $z\\lt\\mu$ または $z=\\mu+q\\cdot\\lambda+r$ とでき、後者のとき
/// $a^z\\equiv a^{\\mu+r}\\pmod{n}$ が成り立つ。
///
/// ここで、$\\mu\\le\\log\_2(n)$, $\\lambda\\sqsubseteq\\varphi(n)$ である。
/// さらに、任意の $n$ に対して $\\log\_2(n)\\le\\varphi(n)$ なので、$z\\ge\\varphi(n)$
/// ならば $z\\ge\\mu$ とわかる。よって、以下のようにできる。
/// $$ \\begin{aligned}
/// a^z \\equiv \\begin{cases}
/// a^z, & \\text{if }z\\lt \\varphi(n); \\\\
/// a^{(z\\bmod\\varphi(n))+\\varphi(n)}, & \\text{otherwise}.
/// \\end{cases}
/// \\end{aligned} $$
///
/// 直感的には、指数部が $\\varphi(n)$ 以上であればすでに周期の中に入っており、入った後は
/// $\\varphi(n)$ を法として合同かつ $\\varphi(n)$ 以上の値さえ得られれば十分ということである。
///
/// [`dlog`]: fn.dlog.html
/// [Idea]: fn.dlog.html#idea
///
/// ## When $b$ is large
/// 前述のように、${}^b a\\bmod{n}$ を求める際に ${}^{b-1} a$ を法 ${\\varphi(n)}$ で考える。
/// その次は $\\varphi(\\varphi(n)), \\varphi(\\varphi(\\varphi(n))), \\dots$ と続く。
/// $\\varphi^\\star(n)$ 段では考えるべき法が $1$ となり、それより上の段のことは無視できる。
///
/// そこで、$\\varphi^\\star(n)$ を考える。奇素数 $p$ に対して $\\varphi(p^e)=p^{e-1}\\cdot(p-1)$
/// が偶数であることと、$\\varphi(2^e)=2^{e-1}$ であることから、$\\varphi(\\varphi(n))\\lt n/2$
/// が成り立つ。すなわち、$\\varphi^\\star(n)\\le 2\\log(n)$ である[^1]。
///
/// [^1]: ゆるゆるな bound である。実際にはもっと速く減りそう。
///
/// よって、$b\\ge 2\\log(n)$ であれば ${}^{b+1} a\\equiv {}^b a\\pmod{n}$ となる。
///
/// ## Common bugs
/// 繰り返し二乗法で $\\varphi(n)$ 以上か判断しつつ $a^z\\bmod\\varphi(n)$ を求める際、
/// $a^{2^\\bullet}$ が $\\varphi(n)$ 以上になった時点で $a^z\\ge\\varphi(n)$
/// と判断してしまうと、誤検出してしまう場合がある。
/// ```ignore
/// fn mod_pow(mut a: u64, mut b: u64, n: u64) -> u64 {
///     let mut res = 1 % n;
///     let mut large = false;
///     while b > 0 {
///         if b & 1 == 1 {
///             res *= a;
///             if res >= n { res %= n; large = true; }
///         }
///         a *= a;
///         if a >= n { a %= n; large = true; } // !
///         b >>= 1;
///     }
///     if large { res + n } else { res }
/// }
/// ```
/// 最後のループで初めて `a >= n` になると、`res < n` なのに `res + n` が返ってしまう。
/// このような理由により、${}^3 2\\bmod 32 = 0$ としてしまうコードがたくさん提出されている。
/// [$\\bullet$](https://judge.yosupo.jp/submission/4054)
/// [$\\bullet$](https://judge.yosupo.jp/submission/4564)
/// [$\\bullet$](https://judge.yosupo.jp/submission/12501)
/// [$\\bullet$](https://judge.yosupo.jp/submission/18734)
/// [$\\bullet$](https://judge.yosupo.jp/submission/23725)
/// [$\\bullet$](https://judge.yosupo.jp/submission/25108)
/// [$\\bullet$](https://judge.yosupo.jp/submission/28794)
/// [$\\bullet$](https://judge.yosupo.jp/submission/36536)
/// [$\\bullet$](https://judge.yosupo.jp/submission/38102)
/// [$\\bullet$](https://judge.yosupo.jp/submission/39646)
/// [$\\bullet$](https://judge.yosupo.jp/submission/40708)
/// [$\\bullet$](https://judge.yosupo.jp/submission/42416)
///
/// # Complexity
/// $O(\\sqrt{n})$ time.
///
/// 律速は、$\\varphi(n), \\varphi(\\varphi(n)), \\dots$ を求めるパートであり、
/// $O(\\sum\_{i=0}^{\\varphi^\\star(n)} \\sqrt{n/2^i}) = O(\\sqrt{n})$ である。
///
/// # Examples
/// ```
/// use nekolib::math::mod_tetration;
///
/// assert_eq!(mod_tetration(0, 0, 100000), 1);
/// assert_eq!(mod_tetration(0, 1, 100000), 0);
/// assert_eq!(mod_tetration(0, 2, 100000), 1);
/// assert_eq!(mod_tetration(0, 3, 100000), 0);
///
/// assert_eq!(mod_tetration(1, 0, 100000), 1);
/// assert_eq!(mod_tetration(1, 1, 100000), 1);
///
/// assert_eq!(mod_tetration(2, 0, 100000), 1);
/// assert_eq!(mod_tetration(2, 1, 100000), 2);
/// assert_eq!(mod_tetration(2, 2, 100000), 4);
/// assert_eq!(mod_tetration(2, 3, 100000), 16);
/// assert_eq!(mod_tetration(2, 4, 100000), 65536);
///
/// assert_eq!(mod_tetration(2, 4, 65535), 1);
/// ```
///
/// # Notations
/// ${}^b a$ は $a\\uparrow\\uparrow b$ (Knuth's up-arrow notation) や
/// $a\\to b\\to 2$ (Conway chained arrow notation) などとも表記される。
pub fn mod_tetration(a: u64, b: u64, n: u64) -> u64 {
    match (a, b, n) {
        (.., 1) => return 0,
        (_, 0, _) => return 1,
        (_, 1, _) => return a % n,
        _ => match rec(a, b, n) {
            z if z >= n => z - n,
            z => z,
        },
    }
}

fn mod_pow(mut a: u64, mut b: u64, n: u64, mut large: bool) -> u64 {
    let cd = ConstDiv::new(n);
    let mut res = 1;
    let mut large_buf = false;
    while b > 0 {
        if b & 1 == 1 {
            res *= a;
            large |= large_buf;
            if res >= n {
                res = cd.rem(res);
                large = true;
            }
        }
        a *= a;
        if a >= n {
            a = cd.rem(a);
            large_buf = true;
        }
        b >>= 1;
    }
    if large {
        res + n
    } else {
        res
    }
}

fn rec(a: u64, b: u64, n: u64) -> u64 {
    match (a, b, n) {
        (0, ..) => return 1 - b % 2,
        (1, ..) => return 1,
        (.., 1) => return 1,
        (_, 1, _) => return a,
        _ => {
            let phi = euler_phi(n);
            let res = rec(a, b - 1, phi);
            mod_pow(a % n, res, n, res >= phi)
        }
    }
}

#[test]
fn test() {
    // for b in 0..20 {
    //     println!("{}", mod_tetration(3, b, 1000000000));
    // }

    for n in 1..100000 {
        if mod_tetration(2, 2, n) != 4 % n {
            eprintln!("{:?}", (2, 2, n));
        }
        if mod_tetration(2, 3, n) != 16 % n {
            eprintln!("{:?}", (2, 3, n));
        }
        if mod_tetration(2, 4, n) != 65536 % n {
            eprintln!("{:?}", (2, 4, n));
        }
        if mod_tetration(2, 5, n) != mod_pow(2, 65536, n, true) - n {
            eprintln!("{:?}", (2, 5, n));
        }

        if mod_tetration(3, 2, n) != 27 % n {
            eprintln!("{:?}", (3, 2, n));
        }
        if mod_tetration(3, 3, n) != 7_625_597_484_987 % n {
            eprintln!("{:?}", (3, 2, n));
        }
    }
}
