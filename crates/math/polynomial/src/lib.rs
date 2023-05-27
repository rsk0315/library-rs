//! 多項式。

use std::fmt::{self, Debug, Display};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Div, DivAssign, Mul, MulAssign, Neg,
    Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use convolution::{butterfly, butterfly_inv, convolve, NttFriendly};
use modint::{ModIntBase, StaticModInt};

/// 多項式。
///
/// ## Notations
///
/// $\\gdef\\deg{\\operatorname{deg}}$
/// $\\gdef\\dd{\\mathrm{d}}$
/// $\\gdef\\dx{{\\textstyle{\\frac{\\dd}{\\dd x}}}}$
/// $\\gdef\\dy{{\\textstyle{\\frac{\\dd}{\\dd y}}}}$
/// $\\gdef\\qed{\\square}$
///
/// $(f(x), g(x))\\bmod x^n$ を $(f(x)\\bmod x^n, g(x)\\bmod x^n)$ の略記として用いる。
///
/// $f(x) = \\sum\_{i=0}^{n} a\_i x^i$ ($a\_{n}\\neq 0$) に対して $\\deg(f) = n$ とする。
/// ただし、$f(x) = 0$ に対しては $\\deg(f) = -\\infty$ とする。
#[derive(Clone, Eq, PartialEq)]
pub struct Polynomial<M: NttFriendly>(Vec<StaticModInt<M>>);

impl<M: NttFriendly> Display for Polynomial<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return write!(f, "0");
        }

        let mut out = false;
        for (i, &c) in self.0.iter().enumerate().filter(|&(_, c)| c.get() > 0) {
            if out {
                write!(f, " + ")?;
            }
            match (i, c.get()) {
                (0, c) => write!(f, "{}", c)?,
                (1, 1) => write!(f, "x")?,
                (1, c) => write!(f, "{}x", c)?,
                (_, 1) => write!(f, "x^{}", i)?,
                (_, c) => write!(f, "{}x^{}", c, i)?,
            }
            out = true;
        }
        Ok(())
    }
}

impl<M: NttFriendly> Debug for Polynomial<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Polynomial")
            .field("f", &self.0.iter().map(|x| x.get()).collect::<Vec<_>>())
            .field("mod", &M::VALUE)
            .finish()
    }
}

impl<M: NttFriendly> Polynomial<M> {
    /// $f(x) = 0$ を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::math::{Mod998244353, Polynomial};
    /// let f = Polynomial::<Mod998244353>::new();
    /// assert!(f.is_zero());
    /// ```
    ///
    /// ```
    /// use nekolib::math::{Mod998244353, Polynomial};
    /// type Poly = Polynomial<Mod998244353>;
    /// let f = Poly::new();
    /// assert!(f.is_zero());
    /// ```
    pub fn new() -> Self { Self(vec![]) }

    fn normalize(&mut self) {
        if self.0.is_empty() {
            return;
        }
        if let Some(i) = (0..self.0.len()).rev().find(|&i| self.0[i].get() > 0)
        {
            self.0.truncate(i + 1);
        } else {
            self.0.clear();
        }
    }

    #[allow(dead_code)]
    fn recip_naive(&self, len: usize) -> Self {
        if len == 0 {
            return Self(vec![]);
        }

        let mut res = Self(vec![self.0[0].recip()]);
        let mut cur_len = 1;
        while cur_len < len {
            cur_len *= 2;
            // f = (2 - f * res) * res

            let mut self_: Self =
                self.0[..self.0.len().min(cur_len)].to_vec().into();

            let ftwo = Self(vec![StaticModInt::new(2); 2 * cur_len]);
            self_.fft_butterfly(2 * cur_len);
            res.fft_butterfly(2 * cur_len);
            let mut tmp = (&ftwo - (&self_ & &res)) & &res;
            tmp.fft_inv_butterfly(2 * cur_len);

            tmp.truncate(cur_len);
            res.0 = tmp.0;
        }
        res.truncate(len);
        res
    }

    /// $f(x)\\cdot g(x) \\equiv 1\\pmod{x^n}$ なる $g(x) \\bmod x^n$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, -1].into();
    /// let g: Poly = [1; 10].into();
    /// assert_eq!(f.recip(10), g);
    /// ```
    pub fn recip(&self, len: usize) -> Self {
        if len == 0 {
            return Self(vec![]);
        }

        let mut res = Self(vec![self.0[0].recip()]);
        let mut cur_len = 1;
        while cur_len < len {
            cur_len *= 2;

            let mut ff: Self =
                self.0[..self.0.len().min(cur_len)].to_vec().into();
            let mut gg = res.clone();
            ff.0.resize(cur_len, StaticModInt::new(0));
            gg.0.resize(cur_len, StaticModInt::new(0));
            butterfly(&mut ff.0);
            butterfly(&mut gg.0);
            for i in 0..cur_len {
                ff.0[i] *= gg.0[i];
            }
            butterfly_inv(&mut ff.0);
            let iz = StaticModInt::new(cur_len).recip();
            for i in 0..cur_len / 2 {
                ff.0[i] = StaticModInt::new(0);
                ff.0[cur_len / 2 + i] = -ff.0[cur_len / 2 + i] * iz;
            }
            butterfly(&mut ff.0);
            for i in 0..cur_len {
                ff.0[i] *= gg.0[i];
            }
            butterfly_inv(&mut ff.0);
            for i in 0..cur_len / 2 {
                ff.0[i] = res.0[i];
                ff.0[cur_len / 2 + i] *= iz;
            }
            res = ff;
        }
        res.truncated(len)
    }

    /// $f(x)\\bmod x^n$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, 2, 3, 4, 5].into();
    /// let g: Poly = [1, 2, 3].into();
    /// assert_eq!(f.truncated(3), g);
    /// ```
    pub fn truncated(mut self, len: usize) -> Self {
        self.truncate(len);
        self
    }

    /// $f(x)\\bmod x^n$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, 2, 3, 4, 5].into();
    /// let g: Poly = [1, 2, 3].into();
    /// assert_eq!(f.ref_truncated(3), g);
    /// assert_eq!(f.ref_truncated(3), g);
    /// ```
    pub fn ref_truncated(&self, len: usize) -> Self {
        self.0[..len.min(self.0.len())].to_vec().into()
    }

    /// $f(x) \\gets f(x) \\bmod x^n$ で更新する。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let mut f: Poly = [1, 2, 3, 4, 5].into();
    /// let g: Poly = [1, 2, 3].into();
    /// f.truncate(3);
    /// assert_eq!(f, g);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
        self.normalize();
    }

    /// $f(x)^{\\mathrm{R}} \\triangleq x^{\\deg(f)}\\cdot f(1/x)$ を返す。ただし $f(x) = 0$ の場合は $0$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [0, 1, 2].into();
    /// let g: Poly = [2, 1].into();
    /// assert_eq!(f.reversed(), g);
    /// ```
    pub fn reversed(mut self) -> Self {
        self.reverse();
        self
    }

    /// $f(x) \\gets f(x)^{\\mathrm{R}}$ で更新する。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let mut f: Poly = [0, 1, 2].into();
    /// let g: Poly = [2, 1].into();
    /// f.reverse();
    /// assert_eq!(f, g);
    /// ```
    pub fn reverse(&mut self) {
        self.0.reverse();
        self.normalize();
    }

    /// $f\'(x)$ を返す。
    ///
    /// $n = \\deg(f) + 1$ とし、
    /// $f(x) = \\sum\_{i=0}^{n-1} a\_i x^i$ のとき、
    /// $$
    /// \\begin{aligned}
    /// f\'(x) &= \\sum\_{i=1}^{n-1} i\\cdot a\_i x^{i-1} \\\\
    /// &= \\sum\_{i=0}^{n-2} (i+1)\\cdot a\_{i+1} x^i
    /// \\end{aligned}
    /// $$
    /// となる。ただし、$f(x) = 0$ のとき $f\'(x) = 0$ である。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, 1, 1, 1].into();
    /// let g: Poly = [1, 2, 3].into();
    /// assert_eq!(f.differential(), g);
    /// ```
    pub fn differential(mut self) -> Self {
        self.differentiate();
        self
    }

    /// $f(x) \\gets f\'(x)$ で更新する。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let mut f: Poly = [1, 1, 1, 1].into();
    /// let g: Poly = [1, 2, 3].into();
    /// f.differentiate();
    /// assert_eq!(f, g);
    /// ```
    pub fn differentiate(&mut self) {
        if self.0.is_empty() {
            return;
        }
        for i in 1..self.0.len() {
            self.0[i] *= StaticModInt::new(i);
        }
        self.0.remove(0);
    }

    ///
    /// $\\int\_0^x f(t)\\, \\dd{t}$ を返す。
    ///
    /// $n = \\deg(f) + 1$ とし、
    /// $f(x) = \\sum\_{i=0}^{n-1} a\_i x^i$ のとき、
    /// $$
    /// \\begin{aligned}
    /// \\int\_0^x f(t)\\, \\dd{t}
    /// &= \\sum\_{i=0}^{n-1} (i+1)^{-1}\\cdot a\_i x^{i+1} \\\\
    /// &= \\sum\_{i=1}^{n} i\\cdot a\_i x^{i+1}
    /// \\end{aligned}
    /// $$
    /// となる。ただし、$f(x) = 0$ のとき $\\int\_0^t f(t)\\, \\dd{t} = 0$ である。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, 2, 3].into();
    /// let g: Poly = [0, 1, 1, 1].into();
    /// assert_eq!(f.integral(), g);
    /// ```
    ///
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f = Poly::from([1, -1]).recip(4).integral();
    /// let g = Poly::from([0, 1, 499122177, 332748118, 748683265]);
    /// // \Integrate (1/(1-x)) dx = x + 1/2 x^2 + 1/3 x^3 + 1/4 x^4 + ...
    /// assert_eq!(f, g);
    /// ```
    pub fn integral(mut self) -> Self {
        self.integrate();
        self
    }

    /// $f(x) \\gets \\int\_0^x f(t)\\, \\dd{t}$ で更新する。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let mut f: Poly = [1, 2, 3].into();
    /// let g: Poly = [0, 1, 1, 1].into();
    /// f.integrate();
    /// assert_eq!(f, g);
    /// ```
    pub fn integrate(&mut self) {
        if self.0.is_empty() {
            return;
        }
        let n = self.0.len();
        let recip = {
            let m = M::VALUE as u64;
            let mut dp = vec![1_u64; n + 1];
            for i in 2..=n {
                let (q, r) = (m / i as u64, m % i as u64);
                dp[i as usize] = m - q * dp[r as usize] % m;
            }
            dp
        };
        for i in 0..n {
            self.0[i] *= StaticModInt::new(recip[i + 1]);
        }
        self.0.insert(0, StaticModInt::new(0));
    }

    /// $\[x\^0] f(x) = 1$ なる $f$ に対し、$\\log(f(x)) \\bmod x^n$ を返す。
    ///
    /// $\\log(1-f(x)) = -\\sum\_{n=1}^{\\infty} \\frac{f(x)^n}{n}$ などで定義される。
    /// $\\dx\\log(f(x)) = f\'(t)\\cdot f(t)^{-1}$ や
    /// $\\log(f(x)g(x)) = \\log(f(x))+\\log(g(x))$
    /// などが成り立つ。
    ///
    /// また、$\[x\^0]\\log(f(x)) = 0$ となる。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, 1].into();
    /// let g: Poly = [0, 1, 499122176, 332748118, 249561088].into();
    /// // log(1+x) = x - 1/2 x^2 + 1/3 x^3 - 1/4 x^4 + ...
    /// assert_eq!(f.log(5), g);
    /// assert_eq!(f.log(5).differential(), [1, -1, 1, -1].into());
    /// ```
    pub fn log(&self, len: usize) -> Self {
        assert_eq!(self.0[0].get(), 1);

        let mut diff = self.clone().differential();
        diff *= self.recip(len);
        diff.integrate();
        diff.truncate(len);
        diff
    }

    /// $\[x\^0] f(x) = 0$ なる $f$ に対し、$\\exp(f(x)) \\bmod x^n$ を返す。
    ///
    /// $\\exp(f(x)) = \\sum\_{n=0}^{\\infty} \\frac{f(x)^n}{n!}$ によって定義される。
    /// $\\dx \\exp(f(x)) = \\exp(f(x))\\cdot \\dx f(x)$ や
    /// $\\exp(f(x)+g(x)) = \\exp(f(x))\\exp(g(x))$
    /// などが成り立つ。
    ///
    /// また、$\\prod\_i f\_i(x) = \\exp(\\sum\_i \\log(f\_i(x)))$
    /// や $\[x\^0] \\exp(f(x)) = 1$
    /// も成り立つ。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [0, 1].into();
    /// let g: Poly = [1, 1, 499122177, 166374059, 291154603].into();
    /// // exp(x) = 1 + x + 1/2 x^2 + 1/6 x^3 + 1/24 x^4 + ...
    /// assert_eq!(f.exp(5), g);
    /// ```
    pub fn exp(&self, len: usize) -> Self {
        assert_eq!(self.0.get(0).map(|x| x.get()).unwrap_or(0), 0);

        if len == 0 {
            return Self(vec![]);
        }

        let mut res = Self(vec![StaticModInt::new(1)]);
        let one = Self(vec![StaticModInt::new(1)]);
        let mut cur_len = 1;
        while cur_len < len {
            cur_len *= 2;
            let mut tmp = &one - res.log(cur_len) + self;
            tmp *= res;
            tmp.truncate(cur_len);
            res = tmp;
        }
        res.truncate(len);
        res
    }

    /// $f(x)\^k \\bmod x^n$ を返す。
    ///
    /// # Ideas
    ///
    /// 自明なケースとして、
    /// $k = 0$ のときは $1$ である。
    /// $f(x) = 0$ のときは $0$ である。$0^0 = 1$ としている。
    ///
    /// それ以外のとき、$f(x) = a\_l x^l \\cdot (1+g(x))$ と書ける。
    /// $$
    /// \\begin{aligned}
    /// f(x)^k &= (a\_l x\^l \\cdot (1+g(x)))^k \\\\
    /// &= a\_l^k x\^{lk} \\cdot \\exp(k\\log(1+g(x)))
    /// \\end{aligned}
    /// $$
    ///
    /// によって計算できる。$\\log$ の引数の定数項が $1$ であることと、$\\exp$
    /// の引数の定数項が $0$ になっていることに注意せよ。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [1, 1].into();
    /// let g: Poly = [1, 4, 6, 4, 1].into();
    /// // (1+x)^4 = 1 + 4x + 6x^2 + 4x^3 + x^4
    /// assert_eq!(f.pow(4, 10), g);
    /// ```
    ///
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [0, 0, 2, 6].into();
    /// let g = Poly::from([64, 1152, 8640, 34560]) << 12;
    /// // (2x^2+6x^3)^6
    /// // = (2x^2 (1 + 3x))^6
    /// // = 64x^12 (1 + 18x + 135x^2 + 540x^3 + ...)
    /// // = 64x^12 + 1152x^13 + 8640x^14 + 34560x^15 + ...
    /// assert_eq!(f.pow(6, 16), g);
    /// ```
    pub fn pow<I: Into<StaticModInt<M>>>(&self, k: I, len: usize) -> Self {
        let k = k.into();
        let k_ = k.get() as usize;

        // 0^0 = 1
        if k_ == 0 {
            return Self::from([1]).truncated(len);
        } else if self.is_zero() {
            return Self::new();
        }

        // f(x) = (a_l x^l) (1+g(x))
        let l = (0..).find(|&i| self.0[i].get() != 0).unwrap();
        let a_l = self.0[l];
        if len <= l * k_ {
            return Self::new();
        }

        let g = (self >> l) / a_l;
        let g_pow = (g.log(len) * k).exp(len - l * k_);
        (g_pow << (l * k_)) * a_l.pow(k_ as u64)
    }

    #[allow(dead_code)]
    fn circular_naive(&self, im: &Self, len: usize) -> (Self, Self) {
        let re = self;
        assert_eq!(re.get(0).get(), 0);
        assert_eq!(im.get(0).get(), 0);
        if len == 0 {
            return (Self::new(), Self::new());
        }

        let one = StaticModInt::new(1);
        let mut cos = Self::from([1]);
        let mut sin = Self::from([0]);
        let mut cur_len = 1;
        while cur_len < len {
            cur_len *= 2;

            let dcos = cos.clone().differential();
            let dsin = sin.clone().differential();

            let hypot = (&cos * &cos + &sin * &sin).recip(cur_len);
            let ecos = &dcos * &cos + &dsin * &sin;
            let esin = &dsin * &cos - &dcos * &sin;

            let logcos = (ecos * &hypot).truncated(cur_len - 1).integral();
            let logsin = (esin * &hypot).truncated(cur_len - 1).integral();

            let gcos = -logcos + one + re.ref_truncated(cur_len);
            let gsin = -logsin + im.ref_truncated(cur_len);
            let hcos = ((&cos * &gcos) - (&sin * &gsin)).truncated(cur_len);
            let hsin = ((&cos * &gsin) + (&sin * &gcos)).truncated(cur_len);

            cos = hcos;
            sin = hsin;
        }

        (cos.truncated(len), sin.truncated(len))
    }

    /// $\[x^0] f(x) = 0$ かつ $\[x^0] g(x) = 0$ なる $h(x) = f(x)+ig(x)$ に対して
    /// $(\\cos(h(x)), \\sin(h(x))) \\bmod x^n$ を返す。
    ///
    /// $\\exp(f(x) + ig(x)) = \\exp(f(x))\\cdot(\\cos(g(x)) + i\\sin(g(x)))$
    /// から定義される。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let zero = Poly::new();
    /// let f: Poly = [0, 1].into();
    /// let g_re: Poly = [1, 0, -499122177, 0, 291154603, 0].into();
    /// let g_im: Poly = [0, 1, 0, -166374059, 0, 856826403].into();
    /// // cos(x) = 1 - 1/2 x^2 + 1/24 x^4 - ...
    /// // sin(x) = x - 1/6 x^3 + 1/120 x^5 - ...
    /// assert_eq!(zero.circular(&f, 6), (g_re, g_im));
    /// ```
    pub fn circular(&self, im: &Self, len: usize) -> (Self, Self) {
        let re = self;
        assert_eq!(re.get(0).get(), 0);
        assert_eq!(im.get(0).get(), 0);
        if len == 0 {
            return (Self::new(), Self::new());
        }

        let one = StaticModInt::new(1);
        let mut cos = Self::from([1]);
        let mut sin = Self::from([0]);
        let mut cur_len = 1;
        while cur_len < len {
            cur_len *= 2;

            let mut dcos = cos.clone().differential();
            let mut dsin = sin.clone().differential();
            cos.fft_butterfly(cur_len);
            sin.fft_butterfly(cur_len);
            dcos.fft_butterfly(cur_len);
            dsin.fft_butterfly(cur_len);

            let mut hypot = (&cos & &cos) + (&sin & &sin);
            let mut ecos = (&dcos & &cos) + (&dsin & &sin);
            let mut esin = (&dsin & &cos) - (&dcos & &sin);
            hypot.fft_inv_butterfly(cur_len);
            hypot = hypot.recip(cur_len);
            hypot.fft_butterfly(2 * cur_len);
            ecos.fft_butterfly_double(2 * cur_len);
            esin.fft_butterfly_double(2 * cur_len);

            let mut logcos = &ecos & &hypot;
            let mut logsin = &esin & &hypot;
            logcos.fft_inv_butterfly(2 * cur_len);
            logsin.fft_inv_butterfly(2 * cur_len);
            logcos = logcos.truncated(cur_len - 1).integral();
            logsin = logsin.truncated(cur_len - 1).integral();

            let mut gcos = -logcos + one + re.ref_truncated(cur_len);
            let mut gsin = -logsin + im.ref_truncated(cur_len);
            gcos.fft_butterfly(2 * cur_len);
            gsin.fft_butterfly(2 * cur_len);
            cos.fft_butterfly_double(2 * cur_len);
            sin.fft_butterfly_double(2 * cur_len);

            let mut hcos = (&cos & &gcos) - (&sin & &gsin);
            let mut hsin = (&cos & &gsin) + (&sin & &gcos);
            hcos.fft_inv_butterfly(2 * cur_len);
            hsin.fft_inv_butterfly(2 * cur_len);

            cos = hcos.truncated(cur_len);
            sin = hsin.truncated(cur_len);
        }

        (cos.truncated(len), sin.truncated(len))
    }

    /// $\\cos(f(x)) \\bmod x^n$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let zero = Poly::new();
    /// let f: Poly = [0, 1].into();
    /// let g: Poly = [1, 0, -499122177, 0, 291154603, 0].into();
    /// // cos(x) = 1 - 1/2 x^2 + 1/24 x^4 - ...
    /// assert_eq!(f.cos(6), g);
    /// ```
    pub fn cos(&self, len: usize) -> Self { Self::new().circular(self, len).0 }

    /// $\\sin(f(x)) \\bmod x^n$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let zero = Poly::new();
    /// let f: Poly = [0, 1].into();
    /// let g: Poly = [0, 1, 0, -166374059, 0, 856826403].into();
    /// // sin(x) = x - 1/6 x^3 + 1/120 x^5 - ...
    /// assert_eq!(f.sin(6), g);
    /// ```
    pub fn sin(&self, len: usize) -> Self { Self::new().circular(self, len).1 }

    /// $\\tan(f(x)) \\bmod x^n$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let zero = Poly::new();
    /// let f: Poly = [0, 1].into();
    /// let g: Poly = [0, 1, 0, 332748118, 0, 732045859].into();
    /// // tan(x) = x + 1/3 x^3 + 2/15 x^5 ...
    /// assert_eq!(f.tan(6), g);
    /// ```
    pub fn tan(&self, len: usize) -> Self {
        let (cos, sin) = Self::new().circular(self, len);
        (sin * cos.recip(len)).truncated(len)
    }

    // f(y) = f(y0) + (y-y0) f'(y0) = 0
    // y = y0 - f(y0)/f'(y0)
    /// `self` を初期解とし、$f(y) = 0$ を満たす $y$ を求める。
    ///
    /// `f_dfr` は $(y, n)$ に対して $f(y)\\cdot f\'(y)^{-1} \\bmod x^n$ を返すとする。
    ///
    /// Newton 法による
    /// $$y\_{k+1} = (y\_k - f(y\_k)\\cdot f\'(y\_k)^{-1}) \\bmod x^{2^k}$$
    /// に基づき、
    /// $$y\\xleftarrow{-} (f(y)\\cdot f\'(y)^{-1}) \\bmod x^{2^k}$$
    /// で更新する。
    ///
    /// # Ideas
    /// 多項式 $\\varphi(y)$ の $g$ のまわりでの Taylor 展開は、
    /// $$ \\varphi(y) = \\sum\_{i=0}^{\\deg(\\varphi)} \\varphi\_i\\cdot (y-g)^i $$
    /// として定義される。各係数 $\\varphi\_i$ は一意に定まり、Taylor 係数と呼ばれる。
    ///
    /// 微分して $y=g$ を代入することなどで、ある多項式 $\\psi$ を用いて以下のように書ける。
    /// $$ \\varphi(y) = \\varphi(g) + \\left(\\dy\\varphi(g)\\right)\\cdot (y-g) + \\psi(y)\\cdot (y-g)^2. $$
    ///
    /// さて、$f(y\_k)\\equiv 0 \\pmod{x^{2^k}}$ なる $y\_k$ が得られており、かつ $\\dy f(y\_k)$ が逆元を持つとき、
    /// $$ y\_{k+1} \\triangleq y\_k - f(y\_k)\\cdot \\dy f(y\_k)^{-1} \\bmod {x^{2^{k+1}}} $$
    /// で得られる $y\_{k+1}$ によって $f(y\_{k+1}) \\equiv 0\\pmod{x^{2^{k+1}}}$ が成り立つことを示す。
    ///
    /// ## Proof
    /// まず、$f(y\_k) \\equiv \\pmod{x^{2^k}}$ であることと、$x^{2^k}$ が $x^{2^{k+1}}$ を割り切ることから
    /// $y\_{k+1} = y\_k \\pmod{x^{2^k}}$ は成り立つ。
    /// これより、$(y\_{k+1} - y\_k)^2 \\pmod {x^{2^{k+1}}}$ も従う。
    /// さて、多項式 $f$ の $y\_k$ のまわりでの Taylor 展開から、ある $\\psi$ に対して
    /// $$ f(y) = f(y\_k) + \\left(\\dy f(y\_k)\\right)\\cdot (y-y\_k) + \\psi(y)\\cdot (y-y\_k)^2 $$
    /// が成り立つので、
    /// $$
    ///  f(y) \\equiv f(y\_k) + \\left(\\dy f(y\_k)\\right)\\cdot (y-y\_k) \\equiv 0 \\pmod {x^{2^{k+1}}}
    /// $$
    /// となる。$y$ について整理して
    /// $$
    /// y \\equiv y\_k -f(y\_k)\\cdot \\dy f(y\_k)^{-1} \\pmod{x^{2^{k+1}}}
    /// $$
    /// を得る。$\\qed$
    ///
    /// なお、一般に、環 $R$ において、$x\\in R$ が $y\\in R$ を法とする逆元を持つことは、
    /// $y^i$ ($i\\in\\N\_{\\ge 1}$) を法とする逆元を持つことと同値である。
    ///
    /// よって、上記の手続きを繰り返すことにより、$y$ を任意の次数で求めることができる。
    /// $x^{2^k}\\to x^{2^{k+1}}$ としていた箇所は、一般に $x^l\\to x^{2l}$ と置き換えることも可能。
    /// 実際には、定数項のみを与え、$x^{2^0}=x$ を法として始めることが多いであろう。
    ///
    /// # References
    ///
    /// - Von Zur Gathen, Joachim, and Jürgen Gerhard. *Modern computer algebra*. Cambridge university press, 2013.
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let one = Poly::from([1]);
    /// let two = Poly::from([2]);
    /// let three = Poly::from([3]);
    /// let catalan = |y: &Poly, n| {
    ///     // c(x) = 1 + x c(x)^2
    ///     // f(y) = x y^2 - y + 1
    ///     // f(y) / f'(y) = (x y^2 - y + 1) / (2xy - 1)
    ///     let f = ((y * y) << 1) - y + &one;
    ///     let df = ((y * &two) << 1) - &one;
    ///     (f.truncated(n) * df.recip(n)).truncated(n)
    /// };
    /// let f = Poly::from([1]).polyeqn(6, catalan);
    /// let g = Poly::from([1, 1, 2, 5, 14, 42]);
    /// assert_eq!(f, g);
    /// ```
    pub fn polyeqn(
        mut self,
        n: usize,
        f_dfr: impl Fn(&Self, usize) -> Self, // f(y0)/f'(y0)
    ) -> Self {
        if self.0.is_empty() {
            self.0.push(StaticModInt::new(0));
        }
        let mut d = self.0.len();
        let mut y = self;
        while d < n {
            d *= 2;
            y -= f_dfr(&y, d).truncated(d);
        }
        y.truncated(n)
    }

    // y' = f(y)
    /// `self` を初期解とし、$y\' = f(y, x)$ を満たす $y(x)$ を求める。
    ///
    /// `f_df` は $(y, n)$ に対して $(f(y, x), f\'(y, x)) \\bmod x^n$ を返すとする。
    ///
    /// # Ideas
    ///
    /// 基本的な方針は Newton 法と同じである。Taylor 展開を用いて二次収束する更新式を得る。
    ///
    /// $y\\equiv y\_k \\pmod{x^{2^k}}$ を満たす
    /// $y\_k = \\sum\_{i=0}^{2^k-1} a\_i x^i$ が得られているとする。
    /// このとき、ある $\\psi(y)$ が存在して、$f(y, x)$ の $y\_k$ のまわりでの Taylor 展開が
    /// $$ f(y, x) = f(y\_k, x) + \\left(\\dy f(y\_k, x)\\right)\\cdot (y-y\_k) + \\psi(y)\\cdot (y-y\_k)^2 $$
    /// と書ける。仮定より $y-y\_k\\equiv 0\\pmod{x^{2^k}}$ なので、
    /// $$ f(y, x) \\equiv f(y\_k, x) + \\left(\\dy f(y\_k, x)\\right)\\cdot (y-y\_k) \\pmod{x^{2^{k+1}}} $$
    /// となる。また、$y\' = y\_k\' + (y\' - y\_k\')$ と書けるので、$y\' = f(y, x)$ より
    /// $$ y\_k\' + (y\' - y\_k\') \\equiv f(y\_k, x) + \\left(\\dy f(y\_k, x)\\right)\\cdot (y-y\_k) \\pmod{x^{2^{k+1}}} $$
    /// が成り立つ。
    ///
    /// ここで $e\_k = y-y\_k$ とおくと、
    /// $$ y\_k\' + e\_k\' \\equiv f(y\_k, x) + \\left(\\dy f(y\_k, x)\\right)\\cdot e\_k \\pmod{x^{2^{k+1}}} $$
    /// が成り立つ。$e\_k$ について整理して
    /// $$ e\_k\' + \\underbrace{\\left(-\\dy f(y\_k, x)\\right)}\_{g(x)}\\cdot e\_k \\equiv \\underbrace{f(y\_k, x) - y\_k\'\\vphantom{\\left(\\dy\\right)}}\_{h(x)} \\pmod{x^{2^{k+1}}} $$
    /// を得る。$e\_k\' + g(x)\\cdot e\_k \\equiv h(x)$ の形式の微分方程式が得られたので、これについて考える。
    ///
    /// $\\mu(x) = \\exp(\\int\_0^x g(t)\\, \\dd{t})$ を両辺に掛けて[^intexp]、
    /// $$
    /// \\begin{aligned}
    /// e\_k\'\\cdot\\mu(x) + g(x)\\cdot e\_k\\cdot\\mu(x)
    /// &\\equiv h(x) \\mu(x) \\\\
    /// \\dx \\left(e\_k\\cdot\\mu(x)\\right)
    /// &\\equiv h(x) \\mu(x) \\\\
    /// % e\_k\\cdot \\mu(x) &\\equiv \\int\_0^x h(t)\\mu(t)\\, \\dd{t} + C \\\\
    /// % e\_k &\\equiv \\frac{1}{\\mu(x)}\\left(\\int\_0^x h(t)\\mu(t)\\, \\dd{t} + C\\right) \\\\
    /// \\end{aligned}
    /// $$
    /// より、
    /// $$
    /// e\_k \\equiv \\frac{1}{\\mu(x)}\\left(\\int\_0^x h(t)\\mu(t)\\, \\dd{t} + C\\right) \\pmod{x^{2^{k+1}}}
    /// $$
    /// を得る。$\\exp$ の性質から $\\mu(x) \\equiv 1 \\pmod{x}$ であり、$C\\mu(x)^{-1} \\equiv C\\pmod{x}$ となる。
    /// ところで、$e\_k = y-y\_k\\equiv 0 \\pmod{x^{2^k}}$ であったため、$C = 0$ となる必要がある。
    ///
    /// [^intexp]: $\\exp$ の引数の定数項は $0$ となる必要がある。
    ///
    /// さて、$y = y\_k + e\_k \\pmod{x^{2^{k+1}}}$ なので、$y\_{k+1} \\triangleq y\_k + e\_k$ とすると、$y \\equiv y\_{k+1} \\pmod{x^{2^{k+1}}}$ を得られる。
    /// すなわち、以下で更新することになる。
    ///
    /// $$
    /// \\begin{aligned}
    /// g\_k &= -\\dy f(y\_k, x) \\bmod x^{2^{k+1}} \\\\
    /// \\mu\_k &= \\exp\\left(\\int\_0^x g(t)\\, \\dd{t}\\right) \\bmod x^{2^{k+1}}\\\\
    /// e\_k &= \\frac{1}{\\mu\_k}\\int\_0^x \\big(f(y\_k, x)-y\_k\')\\cdot \\mu\_k\\big)\\, \\dd{x} \\bmod x^{2^{k+1}} \\\\
    /// y\_{k+1} &= y\_k + e\_k
    /// \\end{aligned}
    /// $$
    ///
    /// 実際には $y$ を immutable で管理して $y\\xleftarrow{+}e\_k$ の更新をしている。
    ///
    /// # References
    ///
    /// - Fateman, Richard J. "Series solutions of algebraic and differential equations: a comparison of linear and quadratic algebraic convergence." In *Proceedings of the ACM-SIGSAM 1989 international symposium on Symbolic and algebraic computation*, pp. 11--16. 1989.
    /// - Von Zur Gathen, Joachim, and Jürgen Gerhard. *Modern computer algebra*. Cambridge university press, 2013.
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let x: Poly = [0, 1].into();
    /// let one: Poly = [1].into();
    /// let three: Poly = [3].into();
    /// let f_df = |y: &Poly, n| {
    ///     let d = y - &x;
    ///     // (f(y), f'(y)) = ((y-x)^3+1, 3(y-x)^2)
    ///     let dd = (&d * &d).truncated(n);
    ///     ((&dd * &d + &one).truncated(n), &dd * &three)
    /// };
    ///
    /// let n = 4;
    /// let y = Poly::from([2]).fode(n + 1, f_df);
    ///
    /// // y = x + 2/sqrt(1-8x) = 2 + 9x + 48x^2 + 320x^3 + 2240x^4 + ...
    /// assert_eq!(y, [2, 9, 48, 320, 2240].into());
    /// assert_eq!(f_df(&y, n).0, y.differential());
    /// ```
    pub fn fode(
        mut self,
        n: usize,
        f_df: impl Fn(&Self, usize) -> (Self, Self),
    ) -> Self {
        if self.0.is_empty() {
            self.0.push(StaticModInt::new(0));
        }
        let mut d = self.0.len();
        let mut y = self;
        while d < n {
            d *= 2;
            let (f, df) = f_df(&y, n);
            let h = f - y.clone().differential();
            let u = (-df).integral().exp(n);
            y += (u.recip(n) * (u * h).integral()).truncated(d);
        }
        y.truncated(n)
    }

    /// $\[x^i] f(x)$ を返す。
    ///
    /// # Examples
    /// ```
    /// # use nekolib::math::{Mod998244353, ModIntBase, Polynomial};
    /// # type Poly = Polynomial::<nekolib::math::Mod998244353>;
    /// let f: Poly = [5, 0, 7].into();
    /// assert_eq!(f.get(0).get(), 5);
    /// assert_eq!(f.get(1).get(), 0);
    /// assert_eq!(f.get(2).get(), 7);
    /// assert_eq!(f.get(3).get(), 0);
    /// assert_eq!(f.get(4).get(), 0);
    /// ```
    pub fn get(&self, i: usize) -> StaticModInt<M> {
        self.0.get(i).copied().unwrap_or(StaticModInt::new(0))
    }

    pub fn eval(&self, t: impl Into<StaticModInt<M>>) -> StaticModInt<M> {
        let t = t.into();
        let mut ft = StaticModInt::new(0);
        for &a in self.0.iter().rev() {
            ft *= t;
            ft += a;
        }
        ft
    }

    /// $(\[x^i] f(x))\_{i=0}^{\\deg(f)}$ を返す。
    pub fn into_inner(self) -> Vec<StaticModInt<M>> { self.0 }

    /// $F\_{\\omega}\[f]$ を返す。
    ///
    /// $F$ とか $\\omega$ とかの定義をちゃんと書く。butterfly をどう書くか悩ましい。
    pub fn fft_butterfly(&mut self, len: usize) {
        let ceil_len = len.next_power_of_two();
        self.0.resize(ceil_len, StaticModInt::new(0));
        butterfly(&mut self.0);
        self.normalize();
    }

    /// $F\_{\\omega}^{-1}\[f]$ を返す。
    pub fn fft_inv_butterfly(&mut self, len: usize) {
        let ceil_len = len.next_power_of_two();
        self.0.resize(ceil_len, StaticModInt::new(0));
        butterfly_inv(&mut self.0);
        self.0.truncate(len);
        let iz = StaticModInt::new(ceil_len).recip();
        for c in &mut self.0 {
            *c *= iz;
        }
        self.normalize();
    }

    /// $F\_{\\omega\^2}\[f]$ を $F\_{\\omega}\[f]$ で更新する。
    // [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]
    pub fn fft_butterfly_double(&mut self, to_len: usize) {
        if self.is_zero() {
            return;
        }

        let mut dbl = self.clone();
        let g = StaticModInt::<M>::new(M::PRIMITIVE_ROOT);
        let zeta = g.pow((M::VALUE as u64 - 1) / (to_len as u64));

        dbl.fft_inv_butterfly(to_len / 2);
        let mut r = StaticModInt::new(1);
        for i in 0..dbl.0.len() {
            dbl.0[i] *= r;
            r *= zeta;
        }
        dbl.fft_butterfly(to_len / 2);
        self.0.resize(to_len / 2, StaticModInt::new(0));
        self.0.append(&mut dbl.0);
    }

    /// $f(x) = 0$ を返す。
    pub fn is_zero(&self) -> bool { self.0.is_empty() }

    /// $\\deg(f)-1$ を返す。ただし $f(x) = 0$ のときは $0$ を返す。
    pub fn len(&self) -> usize { self.0.len() }

    /// $(f(x) / g(x), f(x) \\bmod g(x))$ を返す。
    ///
    /// $f(x) / g(x)$ は $f(x)\\cdot g(x)^{-1}$ ではなく多項式としての除算である。
    pub fn div_mod(&self, other: &Polynomial<M>) -> (Self, Self) {
        let q = self / other;
        let r = self - &q * other;
        (q, r)
    }

    // [x^n] self/other
    /// $\[x^n] f(x) \\cdot g(x)^{-1}$ を返す。
    pub fn div_nth(
        &self,
        other: &Polynomial<M>,
        mut n: usize,
    ) -> StaticModInt<M> {
        let mut p = self.clone();
        let mut q = other.clone();
        while n > 0 {
            let d = (2 * q.0.len() - 1).next_power_of_two();
            p.fft_butterfly(d);
            q.fft_butterfly(d);
            let pq_: Vec<_> = (0..d).map(|i| p.get(i) * q.get(i ^ 1)).collect();
            let qq_: Vec<_> =
                (0..d).step_by(2).map(|i| q.get(i) * q.get(i + 1)).collect();
            let (mut pq_, mut qq_): (Self, Self) = (pq_.into(), qq_.into());
            pq_.fft_inv_butterfly(d);
            qq_.fft_inv_butterfly(d / 2);
            let u: Vec<_> = (n % 2..d).step_by(2).map(|i| pq_.get(i)).collect();
            p = u.into();
            q = qq_.into();
            n /= 2;
        }
        p.get(0)
    }
}

impl<M: NttFriendly> From<Vec<StaticModInt<M>>> for Polynomial<M> {
    fn from(buf: Vec<StaticModInt<M>>) -> Self {
        let mut res = Self(buf);
        res.normalize();
        res
    }
}

impl<'a, M: NttFriendly> From<&'a [StaticModInt<M>]> for Polynomial<M> {
    fn from(buf: &'a [StaticModInt<M>]) -> Self {
        let mut res = Self(buf.to_vec());
        res.normalize();
        res
    }
}

impl<M: NttFriendly, const N: usize> From<[StaticModInt<M>; N]>
    for Polynomial<M>
{
    fn from(buf: [StaticModInt<M>; N]) -> Self {
        let mut res = Self(buf.to_vec());
        res.normalize();
        res
    }
}

macro_rules! impl_from {
    ( $($ty:ty) * ) => { $(
        impl<M: NttFriendly> From<Vec<$ty>> for Polynomial<M> {
            fn from(buf: Vec<$ty>) -> Self {
                let mut res =
                    Self(buf.into_iter().map(StaticModInt::new).collect());
                res.normalize();
                res
            }
        }
        impl<'a, M: NttFriendly> From<&'a [$ty]> for Polynomial<M> {
            fn from(buf: &'a [$ty]) -> Self {
                let mut res =
                    Self(buf.iter().map(|&x| StaticModInt::new(x)).collect());
                res.normalize();
                res
            }
        }
        impl<M: NttFriendly, const N: usize> From<[$ty; N]> for Polynomial<M> {
            fn from(buf: [$ty; N]) -> Self {
                let mut res =
                    Self(buf.iter().map(|&x| StaticModInt::new(x)).collect());
                res.normalize();
                res
            }
        }
    )* }
}

impl_from! {
    i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize
}

// Polynomial<M> @= Polynomial<M>

impl<'a, M: NttFriendly> AddAssign<&'a Polynomial<M>> for Polynomial<M> {
    fn add_assign(&mut self, other: &'a Polynomial<M>) {
        let n = self.0.len().max(other.0.len());
        self.0.resize(n, StaticModInt::new(0));
        for i in 0..other.0.len() {
            self.0[i] += other.0[i];
        }
        self.normalize();
    }
}

impl<M: NttFriendly> AddAssign for Polynomial<M> {
    fn add_assign(&mut self, other: Polynomial<M>) { self.add_assign(&other); }
}

impl<'a, M: NttFriendly> SubAssign<&'a Polynomial<M>> for Polynomial<M> {
    fn sub_assign(&mut self, other: &'a Polynomial<M>) {
        let n = self.0.len().max(other.0.len());
        self.0.resize(n, StaticModInt::new(0));
        for i in 0..other.0.len() {
            self.0[i] -= other.0[i];
        }
        self.normalize();
    }
}

impl<M: NttFriendly> SubAssign for Polynomial<M> {
    fn sub_assign(&mut self, other: Polynomial<M>) { self.sub_assign(&other); }
}

impl<'a, M: NttFriendly> MulAssign<&'a Polynomial<M>> for Polynomial<M> {
    fn mul_assign(&mut self, other: &'a Polynomial<M>) {
        self.mul_assign(other.clone());
    }
}

impl<M: NttFriendly> MulAssign for Polynomial<M> {
    fn mul_assign(&mut self, other: Polynomial<M>) {
        let conv = convolve(std::mem::take(&mut self.0), other.0);
        self.0 = conv;
        self.normalize();
    }
}

impl<'a, M: NttFriendly> DivAssign<&'a Polynomial<M>> for Polynomial<M> {
    fn div_assign(&mut self, other: &'a Polynomial<M>) {
        self.div_assign(other.clone());
    }
}

impl<M: NttFriendly> DivAssign for Polynomial<M> {
    fn div_assign(&mut self, mut other: Polynomial<M>) {
        let deg = self.0.len() - other.0.len();
        self.reverse();
        other.reverse();
        *self *= other.recip(deg + 1);
        self.0.resize(deg + 1, StaticModInt::new(0));
        self.reverse();
    }
}

impl<'a, M: NttFriendly> RemAssign<&'a Polynomial<M>> for Polynomial<M> {
    fn rem_assign(&mut self, other: &'a Polynomial<M>) {
        self.rem_assign(other.clone());
    }
}

impl<M: NttFriendly> RemAssign for Polynomial<M> {
    fn rem_assign(&mut self, other: Polynomial<M>) {
        let div = &*self / &other;
        *self -= div * &other;
    }
}

impl<'a, M: NttFriendly> BitAndAssign<&'a Polynomial<M>> for Polynomial<M> {
    fn bitand_assign(&mut self, other: &'a Polynomial<M>) {
        self.0.truncate(other.0.len());
        for (ai, &bi) in self.0.iter_mut().zip(&other.0) {
            *ai *= bi;
        }
        self.normalize();
    }
}

impl<M: NttFriendly> BitAndAssign for Polynomial<M> {
    fn bitand_assign(&mut self, other: Polynomial<M>) {
        self.bitand_assign(&other);
    }
}

// Polynomial<M> @= StaticModInt<M>

impl<'a, M: NttFriendly> AddAssign<&'a StaticModInt<M>> for Polynomial<M> {
    fn add_assign(&mut self, &other: &'a StaticModInt<M>) {
        if other.get() == 0 {
            return;
        }
        if self.0.is_empty() {
            self.0.push(other);
        } else {
            self.0[0] += other;
        }
        self.normalize();
    }
}

impl<M: NttFriendly> AddAssign<StaticModInt<M>> for Polynomial<M> {
    fn add_assign(&mut self, other: StaticModInt<M>) {
        self.add_assign(&other);
    }
}

impl<'a, M: NttFriendly> SubAssign<&'a StaticModInt<M>> for Polynomial<M> {
    fn sub_assign(&mut self, &other: &'a StaticModInt<M>) {
        if other.get() == 0 {
            return;
        }
        if self.0.is_empty() {
            self.0.push(-other);
        } else {
            self.0[0] -= other;
        }
        self.normalize();
    }
}

impl<M: NttFriendly> SubAssign<StaticModInt<M>> for Polynomial<M> {
    fn sub_assign(&mut self, other: StaticModInt<M>) {
        self.sub_assign(&other);
    }
}

impl<'a, M: NttFriendly> MulAssign<&'a StaticModInt<M>> for Polynomial<M> {
    fn mul_assign(&mut self, &other: &'a StaticModInt<M>) {
        if other.get() == 0 {
            self.0.clear();
            return;
        }
        if self.0.is_empty() {
            return;
        }

        for c in &mut self.0 {
            *c *= other;
        }
        self.normalize();
    }
}

impl<M: NttFriendly> MulAssign<StaticModInt<M>> for Polynomial<M> {
    fn mul_assign(&mut self, other: StaticModInt<M>) {
        self.mul_assign(&other);
    }
}

impl<'a, M: NttFriendly> DivAssign<&'a StaticModInt<M>> for Polynomial<M> {
    fn div_assign(&mut self, &other: &'a StaticModInt<M>) {
        assert_ne!(other.get(), 0);
        if self.0.is_empty() {
            return;
        }

        let other = other.recip();
        for c in &mut self.0 {
            *c *= other;
        }
        self.normalize();
    }
}

impl<M: NttFriendly> DivAssign<StaticModInt<M>> for Polynomial<M> {
    fn div_assign(&mut self, other: StaticModInt<M>) {
        self.div_assign(&other);
    }
}

impl<'a, M: NttFriendly> RemAssign<&'a StaticModInt<M>> for Polynomial<M> {
    fn rem_assign(&mut self, &other: &'a StaticModInt<M>) {
        assert_ne!(other.get(), 0);
        if self.0.is_empty() {
            return;
        }

        self.0.clear();
    }
}

impl<M: NttFriendly> RemAssign<StaticModInt<M>> for Polynomial<M> {
    fn rem_assign(&mut self, other: StaticModInt<M>) {
        self.rem_assign(&other);
    }
}

impl<'a, M: NttFriendly> BitAndAssign<&'a StaticModInt<M>> for Polynomial<M> {
    fn bitand_assign(&mut self, &other: &'a StaticModInt<M>) {
        if self.0.is_empty() {
            return;
        }
        if other.get() == 0 {
            self.0.clear();
        } else {
            self.0.truncate(1);
            self.0[0] *= other;
            self.normalize();
        }
    }
}

impl<M: NttFriendly> BitAndAssign<StaticModInt<M>> for Polynomial<M> {
    fn bitand_assign(&mut self, other: StaticModInt<M>) {
        self.bitand_assign(&other);
    }
}

macro_rules! impl_binop {
    ( $( ($op:ident, $op_assign:ident, $op_trait:ident, $op_assign_trait:ident), )* ) => {
        $(
            impl<'a, M: NttFriendly> $op_trait<Polynomial<M>> for &'a Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(self, other: Polynomial<M>) -> Polynomial<M> {
                    self.clone().$op(other)
                }
            }
            impl<'a, M: NttFriendly> $op_trait<&'a Polynomial<M>> for Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(mut self, other: &'a Polynomial<M>) -> Polynomial<M> {
                    self.$op_assign(other);
                    self
                }
            }
            impl<'a, M: NttFriendly> $op_trait<&'a Polynomial<M>> for &'a Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(self, other: &'a Polynomial<M>) -> Polynomial<M> {
                    self.clone().$op(other)
                }
            }
            impl<M: NttFriendly> $op_trait for Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(mut self, other: Polynomial<M>) -> Polynomial<M> {
                    self.$op_assign(other);
                    self
                }
            }

            impl<'a, M: NttFriendly> $op_trait<StaticModInt<M>> for &'a Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(self, other: StaticModInt<M>) -> Polynomial<M> {
                    self.clone().$op(other)
                }
            }
            impl<'a, M: NttFriendly> $op_trait<&'a StaticModInt<M>> for Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(mut self, other: &'a StaticModInt<M>) -> Polynomial<M> {
                    self.$op_assign(other);
                    self
                }
            }
            impl<'a, M: NttFriendly> $op_trait<&'a StaticModInt<M>> for &'a Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(self, other: &'a StaticModInt<M>) -> Polynomial<M> {
                    self.clone().$op(other)
                }
            }
            impl<M: NttFriendly> $op_trait<StaticModInt<M>> for Polynomial<M> {
                type Output = Polynomial<M>;
                fn $op(mut self, other: StaticModInt<M>) -> Polynomial<M> {
                    self.$op_assign(other);
                    self
                }
            }
        )*
    }
}

impl_binop! {
    (add, add_assign, Add, AddAssign),
    (sub, sub_assign, Sub, SubAssign),
    (mul, mul_assign, Mul, MulAssign),
    (div, div_assign, Div, DivAssign),
    (rem, rem_assign, Rem, RemAssign),
    (bitand, bitand_assign, BitAnd, BitAndAssign),
}

impl<M: NttFriendly> Neg for Polynomial<M> {
    type Output = Polynomial<M>;
    fn neg(mut self) -> Polynomial<M> {
        for c in &mut self.0 {
            *c = -*c;
        }
        self
    }
}

impl<'a, M: NttFriendly> Neg for &'a Polynomial<M> {
    type Output = Polynomial<M>;
    fn neg(self) -> Polynomial<M> { -self.clone() }
}

impl<M: NttFriendly> ShlAssign<usize> for Polynomial<M> {
    fn shl_assign(&mut self, sh: usize) {
        if !self.0.is_empty() {
            self.0.splice(0..0, (0..sh).map(|_| StaticModInt::new(0)));
        }
    }
}

impl<M: NttFriendly> Shl<usize> for Polynomial<M> {
    type Output = Polynomial<M>;
    fn shl(mut self, sh: usize) -> Self::Output {
        self.shl_assign(sh);
        self
    }
}

impl<'a, M: NttFriendly> Shl<usize> for &'a Polynomial<M> {
    type Output = Polynomial<M>;
    fn shl(self, sh: usize) -> Self::Output { self.clone().shl(sh) }
}

impl<M: NttFriendly> ShrAssign<usize> for Polynomial<M> {
    fn shr_assign(&mut self, sh: usize) {
        if !self.0.is_empty() {
            self.0.splice(0..sh.min(self.0.len()), None);
        }
    }
}

impl<M: NttFriendly> Shr<usize> for Polynomial<M> {
    type Output = Polynomial<M>;
    fn shr(mut self, sh: usize) -> Self::Output {
        self.shr_assign(sh);
        self
    }
}

impl<'a, M: NttFriendly> Shr<usize> for &'a Polynomial<M> {
    type Output = Polynomial<M>;
    fn shr(self, sh: usize) -> Self::Output { self.clone().shr(sh) }
}

#[test]
fn sanity_check() {
    type Poly = Polynomial<modint::Mod998244353>;

    let f: Poly = vec![0, 1, 2, 3, 4].into();
    let g = Poly::from(&[0, 1, 2, 4, 8][..]);
    assert_eq!(&f * g, Poly::from([0, 0, 1, 4, 11, 26, 36, 40, 32]));

    let x: Poly = [0, 1].into();
    let exp_recip: Vec<_> =
        x.exp(10).0.into_iter().map(|x| x.recip().get()).collect();
    assert_eq!(exp_recip, [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880]);

    let one_x: Poly = [1, -1].into();
    let log_diff = one_x.log(10).differential();
    assert_eq!(log_diff, Poly::from([-1; 9]));

    let h: Poly = [1, 9, 2, 6, 8, 3].into();
    let x_ten: Poly =
        (0..9).map(|_| 0).chain(Some(1)).collect::<Vec<_>>().into();
    assert_eq!((&h * h.recip(10)) % &x_ten, Poly::from([1]));

    assert_eq!((&f / &x).integral(), &x * Poly::from([1; 4]));

    let x1: Poly = [1; 2].into();
    assert_eq!(x1.pow(5, 10), &x1 * &x1 * &x1 * &x1 * &x1);

    assert_eq!(x1.pow(998244352, 10) * &x1 % &x_ten, x1.pow(998244353, 10));
}

#[test]
fn fft() {
    type Poly = Polynomial<modint::Mod998244353>;

    const N: usize = 4 + 4 + 4 + 1;
    let one: Poly = [1].into();
    let f: Poly = [0, 1, 2, 3, 4].into();
    let g: Poly = [0, 1, 2, 4, 8].into();
    let h: Poly = [0, 6, 5, 4, 3].into();

    let fft = |f: &Poly| {
        let mut f = f.clone();
        f.fft_butterfly(N);
        f
    };
    let ifft = |f: &Poly| {
        let mut f = f.clone();
        f.fft_inv_butterfly(N);
        f
    };

    let fone: Poly = [1; N.next_power_of_two() as usize].into();
    let ff = fft(&f);
    let fg = fft(&g);
    let fh = fft(&h);

    assert_eq!(fft(&(&f + &one)), fft(&f) + &fone);

    assert_eq!(f, ifft(&ff));
    assert_eq!(&f + &h, ifft(&(&ff + &fh)));
    assert_eq!(&f * &g, ifft(&(&ff & &fg)));

    assert_eq!(&f * &g * &h, ifft(&(&ff & &fg & &fh)));
    assert_eq!(f * g + h, ifft(&((ff & fg) + fh)));
}

#[test]
fn recip() {
    type Mi = modint::ModInt998244353;
    type Poly = Polynomial<modint::Mod998244353>;

    let f: Poly = [1, 2, 3, 4].into();
    assert_eq!(f.recip(10), f.recip_naive(10));

    let n = 100;
    let f = Poly::from([1, -1]).recip(n).integral();
    for i in 1..=n {
        assert_eq!((f.get(i) * Mi::new(i)).get(), 1);
    }
}

#[test]
fn pow() {
    type Poly = Polynomial<modint::Mod998244353>;

    let f: Poly = [0, 0, 0, 2, 1, 3].into();

    for len in 0..100 {
        let mut g = Poly::from([1]).truncated(len);
        for k in 0..=10 {
            assert_eq!(f.pow(k, len), g);

            g *= &f;
            g.truncate(len);
        }
    }
}

#[test]
fn polyeqn() {
    type Poly = Polynomial<modint::Mod998244353>;
    type Mi = modint::ModInt998244353;

    let f: Poly = [1, 2, 3, 4, 5].into();
    let n = 10;
    let g = Poly::from([1])
        .polyeqn(n, |y, n| (&f - y.recip(n)) * (y * y).truncated(n));
    assert_eq!(g, f.recip(n));

    let cat = Poly::from([1]).polyeqn(n, |y, n| {
        let f = ((y * y) << 1) - y + Mi::new(1);
        let df = (y << 1) * Mi::new(2) - Mi::new(1);
        (f.truncated(n) * df.recip(n)).truncated(n)
    });
    assert_eq!(cat, Poly::from([1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]));
}

#[test]
fn fode() {
    type Poly = Polynomial<modint::Mod998244353>;
    type Mi = modint::ModInt998244353;

    let one = Mi::new(1);
    let two = Mi::new(2);
    let three = Mi::new(3);
    let x: Poly = [0, 1].into();

    let n = 20;
    let f_df = |y: &Poly, n| {
        let d = y - &x;
        // (f(y), f'(y)) = ((y-x)^2+1, 2(y-x))
        ((&d * &d + one).truncated(n), &d * two)
    };
    let y = Poly::from([1]).fode(n + 1, f_df);

    // f(y) - y' = 0; y = x + 1/(1-x)
    assert_eq!(f_df(&y, n).0, y.differential());

    let f_df = |y: &Poly, n| {
        let d = y - &x;
        // (f(y), f'(y)) = ((y-x)^3+1, 3(y-x))
        let dd = (&d * &d).truncated(n);
        ((&dd * &d + one).truncated(n), &dd * three)
    };
    let y = Poly::from([2]).fode(n + 1, f_df);

    // y = x + 2/sqrt(1-8x) = 2 + 9x + 48x^2 + 320x^3 + ...
    // (2/(y-x))^2 = 1-8x
    assert_eq!(((&y - &x) / two).recip(n).pow(2, n), Poly::from([1, -8]));
    assert_eq!(f_df(&y, n).0, y.differential());

    let catalan = |y: &Poly, n| {
        // (f(y), f'(y)) = (y^2/(1-2xy), 2y(1-xy)/(1-2xy)^2)
        let xy2r = (-((y * Mi::new(2)) << 1) + Mi::new(1)).recip(n);
        let f = ((y * y).truncated(n) * &xy2r).truncated(n);
        let df = (y * Mi::new(2) * (-(y << 1) + Mi::new(1))).truncated(n)
            * (&xy2r * &xy2r).truncated(n);
        (f, df.truncated(n))
    };
    let y = Poly::from([1]).fode(10, catalan);
    assert_eq!(y, [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862].into());
}

#[test]
fn fibonacci() {
    type Poly = Polynomial<modint::Mod998244353>;

    let p: Poly = [1].into();
    let q: Poly = [1, -1, -1].into();

    let n = 10;
    let expected = (&p * q.recip(n)).truncated(n);

    let actual: Vec<_> = (0..n).map(|i| p.div_nth(&q, i)).collect();
    let actual: Poly = actual.into();

    assert_eq!(actual, expected);
}

#[test]
fn butterfly_double() {
    type Poly = Polynomial<modint::Mod998244353>;

    let f: Poly = [1, 2, 3, 4, 5].into();
    let fft = |f: &Poly, n| {
        let mut f = f.clone();
        f.fft_butterfly(n);
        f
    };
    let mut ff8_dbl = fft(&f, 8);
    let ff16 = fft(&f, 16);
    ff8_dbl.fft_butterfly_double(16);
    assert_eq!(ff8_dbl, ff16);
}

#[test]
fn sin_cos() {
    type Mi = modint::ModInt998244353;
    type Poly = Polynomial<modint::Mod998244353>;

    let n = 100;
    let zero: Poly = [0].into();
    let x: Poly = [0, 1].into();

    let exp_x = x.exp(n);
    let (exp, o) = x.circular(&zero, n);

    assert_eq!(exp, exp_x);
    assert_eq!(o, zero);

    let (cos, sin) = zero.circular(&x, n);
    for i in 0..n {
        let sgn = Mi::new(if i / 2 % 2 == 0 { 1 } else { -1 });
        if i % 2 == 0 {
            assert_eq!(cos.get(i), sgn * exp_x.get(i));
            assert_eq!(sin.get(i).get(), 0);
        } else {
            assert_eq!(cos.get(i).get(), 0);
            assert_eq!(sin.get(i), sgn * exp_x.get(i));
        }
    }

    // e^(i(x+x^2)) = e^(ix) e^(ix^2) = (cos(x) + i sin(x)) (cos(x^2) + i sin(x^2))
    // = (cos(x) cos(x^2) - sin(x) sin(x^2)) + i (sin(x) cos(x^2) + cos(x) sin(x^2))
    let z = zero.circular(&Poly::from([0, 1, 1]), n);

    let (cos2, sin2): (Poly, Poly) = {
        let mut cos2 = vec![Mi::new(0); n];
        let mut sin2 = vec![Mi::new(0); n];
        for i in (0..n).step_by(2) {
            cos2[i] = cos.get(i / 2);
            sin2[i] = sin.get(i / 2);
        }
        (cos2.into(), sin2.into())
    };

    assert_eq!(z.0, (&cos * &cos2 - &sin * &sin2).truncated(n));
    assert_eq!(z.1, (&sin * &cos2 + &cos * &sin2).truncated(n));
}
