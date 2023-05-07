//! 多項式。

use std::fmt::{self, Debug, Display};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

use convolution::{convolve, NttFriendly};
use modint::{ModIntBase, StaticModInt};

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

    pub fn recip(&self, len: usize) -> Self {
        if len == 0 {
            return Self(vec![]);
        }

        let mut res = Self(vec![self.0[0].recip()]);
        let two = Self(vec![StaticModInt::new(2)]);
        let mut cur_len = 1;
        while cur_len < len {
            let mut tmp = (&two - self * &res) * &res;
            cur_len *= 2;
            tmp.truncate(cur_len);
            res.0 = tmp.0;
        }
        res.truncate(len);
        res
    }

    pub fn truncated(&self, len: usize) -> Self {
        let mut tmp = self.clone();
        tmp.truncate(len);
        tmp
    }

    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
        self.normalize();
    }

    pub fn reversed(&self) -> Self {
        let mut tmp = self.clone();
        tmp.reverse();
        tmp
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
        self.normalize();
    }

    pub fn differentiate(&mut self) {
        if self.0.is_empty() {
            return;
        }
        for i in 1..self.0.len() {
            self.0[i] *= StaticModInt::new(i);
        }
        self.0.remove(0);
    }

    pub fn differential(mut self) -> Self {
        self.differentiate();
        self
    }

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

    pub fn integral(mut self) -> Self {
        self.integrate();
        self
    }

    pub fn log(&self, len: usize) -> Self {
        assert_eq!(self.0[0].get(), 1);

        let mut diff = self.clone().differential();
        diff *= self.recip(len);
        diff.integrate();
        diff.truncate(len + 1);
        diff
    }

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

    pub fn pow<I: Into<StaticModInt<M>>>(&self, k: I, len: usize) -> Self {
        (self.log(len) * k.into()).exp(len)
    }

    pub fn get(&self, i: usize) -> StaticModInt<M> {
        self.0.get(i).copied().unwrap_or(StaticModInt::new(0))
    }

    pub fn into_inner(self) -> Vec<StaticModInt<M>> { self.0 }
}

impl<M: NttFriendly> From<Vec<StaticModInt<M>>> for Polynomial<M> {
    fn from(buf: Vec<StaticModInt<M>>) -> Self {
        let mut res = Self(buf);
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

#[test]
fn sanity_check() {
    type Poly = Polynomial<modint::Mod998244353>;

    let f: Poly = vec![0, 1, 2, 3, 4].into();
    let g: Poly = vec![0, 1, 2, 4, 8].into();
    assert_eq!(&f * g, Poly::from(vec![0, 0, 1, 4, 11, 26, 36, 40, 32]));

    let x: Poly = vec![0, 1].into();
    let exp_recip: Vec<_> =
        x.exp(10).0.into_iter().map(|x| x.recip().get()).collect();
    assert_eq!(exp_recip, [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880]);

    let one_x: Poly = vec![1, -1].into();
    let log_diff = one_x.log(10).differential();
    assert_eq!(log_diff, Poly::from(vec![-1; 10]));

    let h: Poly = vec![1, 9, 2, 6, 8, 3].into();
    let x_ten: Poly =
        (0..9).map(|_| 0).chain(Some(1)).collect::<Vec<_>>().into();
    assert_eq!((&h * h.recip(10)) % &x_ten, Poly::from(vec![1]));

    assert_eq!((&f / &x).integral(), &x * Poly::from(vec![1; 4]));

    let x1: Poly = vec![1; 2].into();
    assert_eq!(x1.pow(5, 10), &x1 * &x1 * &x1 * &x1 * &x1);

    assert_eq!(x1.pow(998244352, 10) * &x1 % &x_ten, x1.pow(998244353, 10));
}
