use std::fmt::Debug;

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
    pub fn div(&self, z: u64) -> u64 {
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
            assert_eq!(cd.div(a), a / n);
            assert_eq!(cd.rem(a), a % n);
        }
    }
}
