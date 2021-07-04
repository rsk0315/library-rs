//! Chinese remaindering。

use std::fmt::Debug;

use gcd_recip_::gcd_recip;

/// Chinese remaindering。
///
/// $\\gdef{\\lcm}{\\operatorname\*{lcm}}$
/// $x\\equiv r\_i\\pmod{m\_i}$ ($i=0, 1, \\dots$) を条件として与えたとき、
/// それらすべてを満たす $0\\le x\\lt \\lcm\_i m\_i$ は高々一つ存在する。
///
/// 中国剰余定理から、$\\lcm\_i m\_i = \\prod\_i m\_i$ のときは解が必ず存在する。
/// そうでないときは、存在しない場合もある。
///
/// # Examples
/// ```
/// use nekolib::math::Crt;
///
/// let mut crt = Crt::new();
/// crt.equiv_mod(1, 2)
///    .equiv_mod(2, 3);
/// assert_eq!(crt.solve(), Some((5, 6)));
///
/// crt.equiv_mod(7, 12);
/// assert_eq!(crt.solve(), None);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Crt(Option<(u64, u64)>);

impl Crt {
    /// $x\\equiv 0\\pmod{1}$ で初期化する。
    pub fn new() -> Self { Self(Some((0, 1))) }

    /// 条件 $x\\equiv r\\pmod{m}$ を追加する。
    pub fn equiv_mod(&mut self, r: u64, m: u64) -> &mut Self {
        let (r0, m0) = (r as i64, m as i64);
        let (r1, m1) = match self.0 {
            Some((r, m)) => (r as i64, m as i64),
            None => return self,
        };
        let (r0, r1, m0, m1) =
            if m0 < m1 { (r0, r1, m0, m1) } else { (r1, r0, m1, m0) };
        if m0 % m1 == 0 {
            if r0 % m1 != r1 {
                self.0 = None;
            }
            return self;
        }

        let (g, re) = gcd_recip(m0 as u64, m1 as u64);
        let (g, re) = (g as i64, re as i64);
        let u1 = m1 / g;
        if (r1 - r0) % g != 0 {
            self.0 = None;
            return self;
        }
        let x = (r1 - r0) / g % u1 * re % u1;
        let m = m0 * u1;
        let r = match r0 + x * m0 {
            r if r < 0 => r + m,
            r => r,
        };
        self.0 = Some((r as u64, m as u64));
        self
    }

    /// $x\\equiv r\_i\\mod{m\_i}$ なる $0\\le x\\lt \\lcm\_i m\_i$
    /// が存在すれば、その組を返す。
    pub fn solve(&self) -> Option<(u64, u64)> { self.0 }
}

#[test]
fn test_feasible() {
    for n in 0..=1000 {
        let mut crt = Crt::new();
        crt.equiv_mod(n % 2, 2);
        assert_eq!(crt.solve(), Some((n % 2, 2)));
        crt.equiv_mod(n % 16, 16);
        assert_eq!(crt.solve(), Some((n % 16, 16)));
        crt.equiv_mod(n % 30, 30);
        assert_eq!(crt.solve(), Some((n % 240, 240)));
        crt.equiv_mod(n % 7, 7);
        assert_eq!(crt.solve(), Some((n % 1680, 1680)));
    }
}

#[test]
fn test_small() {
    let m = vec![3, 4, 5, 6, 8];
    let lcm = vec![3, 12, 60, 60, 120];
    for r0 in 0..m[0] {
        for r1 in 0..m[1] {
            for r2 in 0..m[2] {
                for r3 in 0..m[3] {
                    for r4 in 0..m[4] {
                        let mut crt = Crt::new();
                        let mut rm = vec![];
                        crt.equiv_mod(r0, m[0]);
                        rm.push((r0, m[0]));
                        assert_eq!(crt.solve(), crt_naive(&rm, lcm[0]));
                        crt.equiv_mod(r1, m[1]);
                        rm.push((r1, m[1]));
                        assert_eq!(crt.solve(), crt_naive(&rm, lcm[1]));
                        crt.equiv_mod(r2, m[2]);
                        rm.push((r2, m[2]));
                        assert_eq!(crt.solve(), crt_naive(&rm, lcm[2]));
                        crt.equiv_mod(r3, m[3]);
                        rm.push((r3, m[3]));
                        assert_eq!(crt.solve(), crt_naive(&rm, lcm[3]));
                        crt.equiv_mod(r4, m[4]);
                        rm.push((r4, m[4]));
                        assert_eq!(crt.solve(), crt_naive(&rm, lcm[4]));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
fn crt_naive(rm: &[(u64, u64)], lcm: u64) -> Option<(u64, u64)> {
    (0..lcm).find(|&x| rm.iter().all(|&(r, m)| x % m == r)).map(|x| (x, lcm))
}
