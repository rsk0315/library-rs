use gcd_recip::GcdRecip;

pub trait CrtMod {
    type I;
    fn crt_mod(&self, m: Self::I) -> Self::I;
}

macro_rules! impl_crt_mod {
    ( $($ty:ty)* ) => { $(
        impl CrtMod for [($ty, $ty)] {
            type I = $ty;
            fn crt_mod(&self, mu: $ty) -> $ty {
                let n = self.len();
                let mut s = vec![0; n];
                for i in 0..n {
                    let (ri, mi) = self[i];
                    let mut prod = 1;
                    let mut sum = 0;
                    for j in 0..i {
                        let mj = self[j].1;
                        sum = (sum + s[j] * prod % mi) % mi;
                        prod = (prod * mj) % mi;
                    }
                    let left = (ri + (mi - sum)) % mi;
                    let right = prod.gcd_recip(mi).1;
                    s[i] = (left * right) % mi;
                }

                let mut prod = 1;
                let mut sum = 0;
                for j in 0..n {
                    let mj = self[j].1;
                    sum = (sum + s[j] * prod % mu) % mu;
                    prod = (prod * mj) % mu;
                }
                sum
            }
        }
    )* };
}

impl_crt_mod! { u8 u16 u32 u64 u128 usize }

pub trait CrtWrapping {
    type I;
    fn crt_wrapping(&self) -> Self::I;
}

macro_rules! impl_crt_wrapping {
    ( $($ty:ty)* ) => { $(
        impl CrtWrapping for [($ty, $ty)] {
            type I = $ty;
            fn crt_wrapping(&self) -> $ty {
                let n = self.len();
                let mut s = vec![0; n];
                for i in 0..n {
                    let (ri, mi) = self[i];
                    let mut prod = 1;
                    let mut sum = 0;
                    for j in 0..i {
                        let mj = self[j].1;
                        sum = (sum + s[j] * prod % mi) % mi;
                        prod = (prod * mj) % mi;
                    }
                    let left = (ri + (mi - sum)) % mi;
                    let right = prod.gcd_recip(mi).1;
                    s[i] = (left * right) % mi;
                }

                let mut prod: $ty = 1;
                let mut sum: $ty = 0;
                for j in 0..n {
                    let mj = self[j].1;
                    sum = sum.wrapping_add(s[j].wrapping_mul(prod));
                    prod = prod.wrapping_mul(mj);
                }
                sum
            }
        }
    )* };
}

impl_crt_wrapping! { u8 u16 u32 u64 u128 usize }

#[test]
fn sanity_check_mod() {
    let a2pow80 = [
        (254739770_u64, 7 << 26 | 1),
        (1481734260, 27 << 26 | 1),
        (1038248692, 15 << 27 | 1),
    ]; // 2^80
    assert_eq!(a2pow80.crt_mod(998244353), 382013690);

    let a0 = [(0_u64, 7 << 26 | 1), (0, 27 << 26 | 1), (0, 15 << 27 | 1)];
    assert_eq!(a0.crt_mod(998244353), 0);
}

#[test]
fn sanity_check_wrapping() {
    let a3pow55 = [
        (285021974_u64, 7 << 26 | 1),
        (723309387, 27 << 26 | 1),
        (1219762234, 15 << 27 | 1),
    ]; // 3^55
    assert_eq!(a3pow55.crt_wrapping(), 12511015583298303947);

    let a0 = [(0_u64, 7 << 26 | 1), (0, 27 << 26 | 1), (0, 15 << 27 | 1)];
    assert_eq!(a0.crt_wrapping(), 0);
}

#[test]
fn large() {
    let large_u64 = [
        (867145189_u64, 1107296257),
        (1121462194, 1711276033),
        (567952613, 2113929217),
        (292122917, 469762049),
        (1550969568, 1811939329),
        (1001085957, 2013265921),
    ]; // (2^64-1)^2 2^25
    assert_eq!(large_u64.crt_mod(998244353), 2258058);
    assert_eq!(large_u64.crt_wrapping(), 1 << 25);

    let large_u128 = [
        (305535025_u128, 754974721),
        (1105782392, 1224736769),
        (1129452415, 2130706433),
        (42581335, 167772161),
        (736341624, 1107296257),
        (937167787, 1711276033),
        (218059526, 2113929217),
        (100381360, 469762049),
        (1394496118, 1811939329),
        (1096317127, 2013265921),
    ]; // (2^128-1)^2 2^24
    assert_eq!(large_u128.crt_mod(998244353), 577639010);
    assert_eq!(large_u128.crt_wrapping(), 1 << 24);
}
