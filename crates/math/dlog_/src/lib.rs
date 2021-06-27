//! 離散対数。

use std::collections::HashMap;

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
/// $0\\le b, a\\lt n$ とする。
/// また、$0^0 = 1$ とする。
///
/// コーナーケースに注意。
/// $n=1$ なら $0^0=1\\equiv 0$ より $z=0$ を返す。
/// 以下、$n\\gt 1$ とする。
/// - $\\mathrm{dlog}_\\bullet(1) = 0$
/// - $\\mathrm{dlog}_0(0) = 1$
///
/// 上記に該当しないとき、$\\mathrm{dlog}_0(\\bullet)$ と $\\mathrm{dlog}_1(\\bullet)$
/// は存在しない。残りのケースについては、以下の方針に基づいて求める。
///
/// # Idea
/// $1\\to b\\to b^2\\to\\dots\\pmod{n}$ でできる functional graph を考えると、$\\rho$
/// の字状になっていることに気づく。しっぽの長さ $\\mu$ は高々 $\\log(n)$、頭の長さ
/// $\\lambda$ は $\\varphi(n)$ の約数になっていることが示せる。
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
/// また、頭の部分に $a$ が含まれるためには $\\gcd(b^\\mu, n)=\\gcd(a, n)$
/// が必要であり、これが成り立たなければ解なしを報告すればよい。
/// よって、以下ではこれが成り立つとする。
///
/// $z = \\mu+i\\cdot\\lfloor\\sqrt{\\lambda}\\rfloor+j$ ($0\\le j\\lt\\sqrt{\\lambda}$)
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
/// $j$ による小さい幅で連想配列を作り、$i$ による大きい幅でそれにアクセスする様子から
/// baby-step giant-step algorithm と呼ばれている。
///
/// ## About $\\mu$ and $\\lambda$
///
/// $\\mu$ と $\\lambda$ を求める方法について述べる。
/// 自然数 $x$ が素数 $p$ で割り切れる回数、すなわち $x$ の素因数分解における $p$
/// の指数を $\\pi\_p(x)$ で表すとする[^2]。
/// $b = \\prod\_{p\_i:\\text{ prime}} p\_i^{e\_i}$ とすると、次が成り立つ。
/// $$ \\mu = \\max\_i \\left\\lceil\\frac{\\pi\_{p\_i}(n)}{e\_i}\\right\\rceil. $$
/// また、$p = \\prod\_i p\_i^{\\pi\_{p\_i}(n)}$, $n\' = n / p$ とする。すなわち、$n$
/// を $b$ の各素因数で割り切れるだけ割ったものを $n\'$ とする。このとき、$\\lambda$
/// は $\\varphi(n\')$ の約数[^3]であり、次の式に基づき愚直に求める[^4]。
/// $$ \\lambda = \\min\_{i\\sqsubseteq \\varphi(n\')} i\\quad\\text{s.t.}\\quad
/// b^\\mu\\cdot b^i \\equiv b^\\mu \\pmod{n}.  $$
/// $\\varphi(n\')$ の約数の個数は $o(\\sqrt{n}/\\log(n))$
/// なので、繰り返し二乗法を用いて $O(\\sqrt{n})$ 時間でできる。
/// 実際には、解が見つかった時点で打ち切れるので、$O(\\lambda + \\sum\_{i\\sqsubseteq
/// \\varphi(n\')}^{i\\le\\lambda} \\log(i))$ 時間になる。前半の $\\lambda$
/// は試し割り法に由来する項なので、約数列挙を $O(1)$ delay でできるなら消去できる。
///
/// [^2]: 標準的な記法があったら知りたいです。
///
/// [^3]: $\\varphi(n)$ と $n\'$ の定義から、$\\varphi(n\')$ は $\\varphi(n)$ の約数であることに注意。
///
/// [^4]: $x\\sqsubseteq y$ で $x$ が $y$ を割り切ることを表す。$x\\mid y$
/// は方向がわかりにくくて嫌い。素因数の多重集合の包含関係のつもり。$\\subseteq$ でもいいかも？
///
/// ## Proof
/// まず、$\\lambda$ が $\\varphi(n\')$ の約数になることを示す。
/// $b^\\mu$ における $p\_i$ の指数は $e\_i\\cdot\\max\_j \\lceil\\pi\_{p\_j}(n)/e\_j\\rceil$
/// であり、特に $e\_i\\cdot \\lceil\\pi\_{p\_i}(n)/e\_i\\rceil \\ge \\pi\_{p\_i}(n)$ 以上である。
/// よって、$b^\\mu$ は $p$ で割り切れ、$b^\\mu \\equiv 0\\pmod{p}$ である。これより、$b^{\\mu+i}
/// \\equiv 0\\pmod{p}$ ($i\\ge 0$) である。
///
/// また、$b$ と $n\'$ は互いに素なので、Euler の定理より $b^{\\varphi(n\')} \\equiv 1\\pmod{n\'}$
/// が成り立つ。よって、$b^{\\mu+\\varphi(n\')}\\equiv b^\\mu \\pmod{n\'}$
/// である。上記の $p$ に関する結果と合わせて $b^{\\mu+\\varphi(n\')}\\equiv b^\\mu \\pmod{n}$
/// を得る。よって、$\\lambda$ が $\\varphi(n\')$ の約数であることが示された。
///
/// 次に、$\\mu$ の値が正しいことを示す。$b^\\mu$ が頭の部分に含まれることは上で示した
/// $b^{\\mu+\\varphi(n\')}\\equiv b^\\mu\\pmod{n}$ より従うので、$\\mu\\gt 0$ のとき
/// $b^{\\mu-1}$ が頭の部分に含まれないことを示せば十分である。
/// $\\mu$ の定義より、ある $i$ に対して $e\_i\\cdot(\\mu-1)\\lt \\pi\_{p\_i}(n)$ が成り立つ。
/// よって $b^{\\mu-1}$ は $p$ は割り切れず、$b^{\\mu-1} \\not\\equiv 0\\pmod{n}$ である。
/// これにより、頭の部分に含まれるどの要素とも等しくないことがわかる。
///
/// 最後に、$\\gcd(b^\\mu, n)=\\gcd(a, n)$ であることが、頭の部分に $a$ が存在する、すなわち
/// ${}^\\exists i\\ge 0: b^{\\mu+i} \\equiv a\\pmod{n}$ であることの必要条件であることを示す。
///
/// - $\\gcd(b^{\\mu+i})\\equiv 0\\pmod{p}$ ($i\\ge 0$)
/// - $\\gcd(b, n\') = 1$ より $\\gcd(b\^{\\mu+i}, n\') = 1$ ($i\\ge 0$)
/// - $n=p\\cdot n\'$
/// - $p$ と $n\'$ は互いに素
///
/// より、$\\gcd(b^{\\mu+i}, n) = p$ ($i\\ge 0$) で一定となる。
/// すなわち、$\\gcd(a, n) = p$ が必要となる。
///
/// # Complexity
/// $O(\\sqrt{n} + \\sqrt{\\lambda}\\cdot H(\\sqrt{\\lambda}))$ time.
/// ただし、$H(n)$ は要素数 $n$ の [`HashMap`] の [`insert`] と [`get`] にかかる時間とする。
///
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`insert`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert
/// [`get`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get
///
/// ここで、$\\lambda$ は最悪 $\\Theta(n)$ である。
/// また、篩の前計算などで $n$, $b$ の素因数分解と $\\varphi(n)$ の約数列挙を $O(1)$ delay
/// でできるなら $\\sqrt{n}$ の項をそれらの個数に落とせる。
///
/// # Implementation notes
/// $\\lambda$ を求める際に $b^{\\mu+i}\\equiv b^\\mu$ なる最小の $i\\sqsubseteq \\varphi(n\')$
/// を探したが、これは Carmichael の $\\lambda$ 関数を用いて $\\lambda(n\')$ と書ける。
/// しかし、実際に $\\lambda(n\')$ を求めるよりも前述のループで求める方が高速だった。
///
/// また、BS/GS パートでは [`BTreeMap`] を用いるよりも [`HashMap`]
/// を用いた方が高速だったので、とりあえずそうしてある。
///
/// [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
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
/// 内部で Euler の $\\varphi$ 関数と約数列挙と素因数分解が必要となるので、
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
/// とはいえ、結局 BS/GS パートが律速になってしまいそう。
///
/// # Naming
/// 関数名は *d*iscrete *log*arithm、引数 $b$ は *b*ase、$a$ は *a*ntilogarithm から。
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

    let mut n_ = n;
    let tail = factors(b)
        .map(|(p, e)| {
            let mut f = 0;
            while n_ % p == 0 {
                n_ /= p;
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
    if n != n_ * gcd_recip(a, n).0 {
        return None;
    }

    let c = divisors(totient_phi(n_))
        .find(|&c| cd.rem(bb * mod_pow_with_cd(b, c, cd)) == bb)
        .unwrap();

    bsgs(bb, b, a, cd, c).map(|head| tail + head)
}

fn bsgs(bb: u64, b: u64, a: u64, cd: ConstDiv, c: u64) -> Option<u64> {
    let step = (1..).find(|&i| i * i * 2 >= c).unwrap();
    let seen = {
        let mut seen = HashMap::new();
        let baby_recip = mod_pow_with_cd(b, c - 1, cd);
        let mut x = a;
        for i in 0..step {
            seen.insert(x, i);
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
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    let n_max = 200;

    for n in 1..=n_max {
        for b in 0..n {
            let expected = {
                let mut expected = HashMap::new();
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
