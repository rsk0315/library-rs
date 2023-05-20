use super::garner;
use super::modint;
use std::cell::RefCell;
use std::thread::LocalKey;

use garner::{CrtMod as CrtModInternal, CrtWrapping};
use modint::{Mod998244353, ModIntBase, Modulus, RemEuclidU32, StaticModInt};

pub struct ButterflyCache<M: NttFriendly> {
    root: Vec<StaticModInt<M>>,
    iroot: Vec<StaticModInt<M>>,
    rate2: Vec<StaticModInt<M>>,
    irate2: Vec<StaticModInt<M>>,
    rate3: Vec<StaticModInt<M>>,
    irate3: Vec<StaticModInt<M>>,
}

const fn primitive_root(p: u32) -> u32 {
    if p == 2 {
        return 1;
    }

    // 2*3*5*7*11*13*17*19*23*29 > 2**32
    let mut divs = [0; 10];
    divs[0] = 2;
    let mut index = 1;
    let mut x = (p - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    let mut d = 3;
    while d <= x / d {
        if x % d == 0 {
            divs[index] = d;
            index += 1;
            while x % d == 0 {
                x /= d;
            }
        }
        d += 2;
    }
    if x > 1 {
        divs[index] = x;
        index += 1;
    }

    let mut g = 2;
    loop {
        let mut ok = true;
        let mut i = 0;
        while i < index {
            if mod_pow(g, (p - 1) / divs[i], p) == 1 {
                ok = false;
                break;
            }
            i += 1;
        }
        if ok {
            return g;
        }
        g += 1;
    }
}

const fn mod_pow(a: u32, mut e: u32, m: u32) -> u32 {
    let mut res = 1;
    let mut a = a as u64;
    let m = m as u64;
    while e > 0 {
        if e & 1 != 0 {
            res = res * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    res as u32
}

pub trait NttFriendly: Modulus {
    const PRIMITIVE_ROOT: u32 = primitive_root(Self::VALUE);
    // ODD << EXP | 1 == MOD
    const EXP: u32 = (Self::VALUE - 1).trailing_zeros();
    const ODD: u32 = Self::VALUE >> Self::EXP;

    fn butterfly_cache()
    -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>>;
}

impl NttFriendly for Mod998244353 {
    fn butterfly_cache()
    -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>> {
        thread_local! {
            static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<Mod998244353>>> = Default::default();
        }
        &BUTTERFLY_CACHE
    }
}

pub fn precompute_butterfly<M: NttFriendly>() -> ButterflyCache<M> {
    let g = StaticModInt::<M>::new(M::PRIMITIVE_ROOT);
    let rank2 = M::EXP as usize;

    let mut root = vec![StaticModInt::new(0); rank2 + 1];
    let mut iroot = vec![StaticModInt::new(0); rank2 + 1];
    root[rank2] = g.pow(M::ODD.into());
    iroot[rank2] = root[rank2].recip();
    for i in (0..rank2).rev() {
        root[i] = root[i + 1] * root[i + 1];
        iroot[i] = iroot[i + 1] * iroot[i + 1];
    }

    let mut rate2 = vec![StaticModInt::new(0); rank2];
    let mut irate2 = vec![StaticModInt::new(0); rank2];
    {
        let mut prod = StaticModInt::new(1);
        let mut iprod = StaticModInt::new(1);
        for i in 0..=rank2 - 2 {
            rate2[i] = root[i + 2] * prod;
            irate2[i] = iroot[i + 2] * iprod;
            prod *= iroot[i + 2];
            iprod *= root[i + 2];
        }
    }

    let mut rate3 = vec![StaticModInt::new(0); rank2];
    let mut irate3 = vec![StaticModInt::new(0); rank2];
    {
        let mut prod = StaticModInt::new(1);
        let mut iprod = StaticModInt::new(1);
        for i in 0..=rank2 - 3 {
            rate3[i] = root[i + 3] * prod;
            irate3[i] = iroot[i + 3] * iprod;
            prod *= iroot[i + 3];
            iprod *= root[i + 3];
        }
    }

    ButterflyCache { root, iroot, rate2, irate2, rate3, irate3 }
}

pub fn butterfly<M: NttFriendly>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { root, rate2, rate3, .. } =
            cache.get_or_insert_with(precompute_butterfly);

        // a[i, i + (n >> len), i + 2 * (n >> len), ...] is transformed
        let mut len = 0;
        while len < h {
            if h - len == 1 {
                let p = 1 << (h - len - 1);
                let mut rot = StaticModInt::new(1);
                for s in 0..1 << len {
                    let offset = s << (h - len);
                    for i in 0..p {
                        let l = a[i + offset];
                        let r = a[i + offset + p] * rot;
                        a[i + offset] = l + r;
                        a[i + offset + p] = l - r;
                    }
                    if s + 1 != 1 << len {
                        rot *= rate2[(!s).trailing_zeros() as usize];
                    }
                }
                len += 1;
            } else {
                // 4-base
                let p = 1 << (h - len - 2);
                let imag_u64 = root[2].get() as u64;
                let mut rot = StaticModInt::new(1);

                for s in 0..1 << len {
                    let rot2 = rot * rot;
                    let rot3 = rot2 * rot;

                    let rot_u64 = rot.get() as u64;
                    let rot2_u64 = rot2.get() as u64;
                    let rot3_u64 = rot3.get() as u64;

                    let offset = s << (h - len);
                    for i in 0..p {
                        let mod2 = (M::VALUE as u64).pow(2);
                        let a0 = a[i + offset].get() as u64;
                        let a1 = a[i + offset + p].get() as u64 * rot_u64;
                        let a2 = a[i + offset + 2 * p].get() as u64 * rot2_u64;
                        let a3 = a[i + offset + 3 * p].get() as u64 * rot3_u64;

                        let a1na3 = StaticModInt::<M>::new(a1 + mod2 - a3);
                        let a1na3imag = a1na3.get() as u64 * imag_u64;
                        let na2 = mod2 - a2;

                        a[i + offset] = StaticModInt::new(a0 + a2 + a1 + a3);
                        a[i + offset + p] =
                            StaticModInt::new(a0 + a2 + (2 * mod2 - (a1 + a3)));
                        a[i + offset + 2 * p] =
                            StaticModInt::new(a0 + na2 + a1na3imag);
                        a[i + offset + 3 * p] =
                            StaticModInt::new(a0 + na2 + (mod2 - a1na3imag));
                    }

                    if s + 1 != 1 << len {
                        rot *= rate3[(!s).trailing_zeros() as usize];
                    }
                }
                len += 2;
            }
        }
    });
}

pub fn butterfly_inv<M: NttFriendly>(a: &mut [StaticModInt<M>]) {
    let n = a.len();
    let h = ceil_pow2(n as u32);

    M::butterfly_cache().with(|cache| {
        let mut cache = cache.borrow_mut();
        let ButterflyCache { iroot, irate2, irate3, .. } =
            cache.get_or_insert_with(precompute_butterfly);

        // a[i, i + (n >> len), i + 2 * (n >> len), ...] is transformed
        let mut len = h;
        while len > 0 {
            if len == 1 {
                let p = 1 << (h - len);
                let mut irot = StaticModInt::new(1);
                for s in 0..1 << (len - 1) {
                    let offset = s << (h - len + 1);
                    for i in 0..p {
                        let l = a[i + offset];
                        let r = a[i + offset + p];
                        a[i + offset] = l + r;
                        a[i + offset + p] = (l - r) * irot
                    }

                    if s + 1 != 1 << (len - 1) {
                        irot *= irate2[(!s).trailing_zeros() as usize];
                    }
                }
                len -= 1;
            } else {
                // 4-base
                let p = 1 << (h - len);
                let mod1 = M::VALUE as u64;
                let iimag_u64 = iroot[2].get() as u64;

                let mut irot = StaticModInt::new(1);
                for s in 0..1 << (len - 2) {
                    let irot2 = irot * irot;
                    let irot3 = irot2 * irot;

                    let irot_u64 = irot.get() as u64;
                    let irot2_u64 = irot2.get() as u64;
                    let irot3_u64 = irot3.get() as u64;

                    let offset = s << (h - len + 2);
                    for i in 0..p {
                        let a0 = a[i + offset].get() as u64;
                        let a1 = a[i + offset + p].get() as u64;
                        let a2 = a[i + offset + 2 * p].get() as u64;
                        let a3 = a[i + offset + 3 * p].get() as u64;

                        let a2na3_u64 =
                            StaticModInt::<M>::new(mod1 + a2 - a3).get() as u64;
                        let a2na3iimag =
                            StaticModInt::<M>::new(a2na3_u64 * iimag_u64);
                        let a2na3iimag_u64 = a2na3iimag.get() as u64;

                        a[i + offset] = StaticModInt::new(a0 + a1 + a2 + a3);
                        a[i + offset + p] = StaticModInt::new(
                            (a0 + (mod1 - a1) + a2na3iimag_u64) * irot_u64,
                        );
                        a[i + offset + 2 * p] = StaticModInt::new(
                            (a0 + a1 + (mod1 - a2) + (mod1 - a3)) * irot2_u64,
                        );
                        a[i + offset + 3 * p] = StaticModInt::new(
                            (a0 + (mod1 - a1) + (mod1 - a2na3iimag_u64))
                                * irot3_u64,
                        );
                    }
                    if s + 1 != 1 << (len - 2) {
                        irot *= irate3[(!s).trailing_zeros() as usize];
                    }
                }
                len -= 2;
            }
        }
    });
}

pub fn convolve<M: NttFriendly>(
    a: Vec<StaticModInt<M>>,
    b: Vec<StaticModInt<M>>,
) -> Vec<StaticModInt<M>> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let (n, m) = (a.len(), b.len());

    if n.min(m) <= 60 { convolve_naive(&a, &b) } else { convolve_fft(a, b) }
}

fn convolve_naive<M: NttFriendly>(
    a: &[StaticModInt<M>],
    b: &[StaticModInt<M>],
) -> Vec<StaticModInt<M>> {
    let (n, m) = (a.len(), b.len());
    let (n, m, a, b) = if n < m { (m, n, b, a) } else { (n, m, a, b) };
    let mut res = vec![StaticModInt::new(0); n + m - 1];
    for i in 0..n {
        for j in 0..m {
            res[i + j] += a[i] * b[j];
        }
    }
    res
}

fn convolve_fft<M: NttFriendly>(
    mut a: Vec<StaticModInt<M>>,
    mut b: Vec<StaticModInt<M>>,
) -> Vec<StaticModInt<M>> {
    let (n, m) = (a.len(), b.len());
    let z = (n + m - 1).next_power_of_two();
    a.resize(z, StaticModInt::new(0));
    b.resize(z, StaticModInt::new(0));

    butterfly(&mut a);
    butterfly(&mut b);

    for (ai, bi) in a.iter_mut().zip(&mut b) {
        *ai *= *bi;
    }
    butterfly_inv(&mut a);

    a.truncate(n + m - 1);
    let iz = StaticModInt::new(z).recip();
    for ai in &mut a {
        *ai *= iz;
    }

    a
}

macro_rules! impl_modint_ntt {
    ( $( ($mod:ident, $val:expr), )* ) => { $(
        #[derive(Clone, Copy, Eq, PartialEq)]
        struct $mod;
        impl Modulus for $mod {
            const VALUE: u32 = $val;
        }
        impl NttFriendly for $mod {
            fn butterfly_cache()
            -> &'static LocalKey<RefCell<Option<ButterflyCache<$mod>>>> {
                thread_local! {
                    static BUTTERFLY_CACHE: RefCell<Option<ButterflyCache<$mod>>> = Default::default();
                }
                &BUTTERFLY_CACHE
            }
        }
    )* }
}

impl_modint_ntt! {
    (Mod45e24p1, 45 << 24 | 1),
    (Mod73e24p1, 73 << 24 | 1),
    (Mod127e24p1, 127 << 24 | 1),
    (Mod5e25p1, 5 << 25 | 1),
    (Mod33e25p1, 33 << 25 | 1),
    (Mod51e25p1, 51 << 25 | 1),
    (Mod63e25p1, 63 << 25 | 1),
    (Mod7e26p1, 7 << 26 | 1),
    (Mod27e26p1, 27 << 26 | 1),
    (Mod15e27p1, 15 << 27 | 1),
}

type Mod0 = Mod15e27p1;
type Mod1 = Mod27e26p1;
type Mod2 = Mod7e26p1;
type Mod3 = Mod63e25p1;
type Mod4 = Mod51e25p1;
type Mod5 = Mod33e25p1;
type Mod6 = Mod5e25p1;
type Mod7 = Mod127e24p1;
type Mod8 = Mod73e24p1;
type Mod9 = Mod45e24p1;

const MOD0: u32 = Mod0::VALUE;
const MOD1: u32 = Mod1::VALUE;
const MOD2: u32 = Mod2::VALUE;
const MOD3: u32 = Mod3::VALUE;
const MOD4: u32 = Mod4::VALUE;
const MOD5: u32 = Mod5::VALUE;
const MOD6: u32 = Mod6::VALUE;
const MOD7: u32 = Mod7::VALUE;
const MOD8: u32 = Mod8::VALUE;
const MOD9: u32 = Mod9::VALUE;

fn convolve_from<M: NttFriendly, I: RemEuclidU32 + Copy>(
    a: &[I],
    b: &[I],
) -> Vec<u32> {
    let a: Vec<_> = a.iter().map(|&ai| StaticModInt::new(ai)).collect();
    let b: Vec<_> = b.iter().map(|&bi| StaticModInt::new(bi)).collect();
    convolve(a, b).into_iter().map(|x: StaticModInt<M>| x.get()).collect()
}

pub fn convolve_u64_acl(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = a.len();
    let m = b.len();

    let mod1 = Mod45e24p1::VALUE as u64;
    let mod2 = Mod5e25p1::VALUE as u64;
    let mod3 = Mod7e26p1::VALUE as u64;
    let m2m3 = mod2 * mod3;
    let m1m3 = mod1 * mod3;
    let m1m2 = mod1 * mod2;
    let m1m2m3 = m1m2.wrapping_mul(mod3);

    type ModInt754974721 = StaticModInt<Mod45e24p1>;
    type ModInt167772161 = StaticModInt<Mod5e25p1>;
    type ModInt469762049 = StaticModInt<Mod7e26p1>;

    let i1 = ModInt754974721::new(m2m3).recip().get() as u64;
    let i2 = ModInt167772161::new(m1m3).recip().get() as u64;
    let i3 = ModInt469762049::new(m1m2).recip().get() as u64;

    let max_bit = 24;
    assert_eq!(mod1 % (1 << max_bit), 1);
    assert_eq!(mod2 % (1 << max_bit), 1);
    assert_eq!(mod3 % (1 << max_bit), 1);
    assert!(n + m - 1 <= (1 << max_bit));

    let c1 = convolve_from::<Mod45e24p1, _>(&a, &b);
    let c2 = convolve_from::<Mod5e25p1, _>(&a, &b);
    let c3 = convolve_from::<Mod7e26p1, _>(&a, &b);

    c1.into_iter()
        .zip(c2)
        .zip(c3)
        .map(|((c1i, c2i), c3i)| {
            let c1i = c1i as u64;
            let c2i = c2i as u64;
            let c3i = c3i as u64;

            let mut x = 0;
            x += (c1i * i1) % mod1 * m2m3;
            x += (c2i * i2) % mod2 * m1m3;
            x += (c3i * i3) % mod3 * m1m2;
            let rem = x.rem_euclid(mod1);
            let diff = if c1i >= rem { c1i - rem } else { mod1 - (rem - c1i) };
            let offset = [0, 0, m1m2m3, 2 * m1m2m3, 3 * m1m2m3];
            x - offset[diff as usize % 5]
        })
        .collect()
}

enum CrtU64 {}
enum CrtWrappingU64 {}
enum CrtU128 {}
enum CrtWrappingU128 {}
#[derive(Copy, Clone)]
struct CrtU32Mod(u32);
#[derive(Copy, Clone)]
struct CrtU64Mod(u64);
#[derive(Copy, Clone)]
struct CrtU128Mod(u128);

type U32x3 = ((u32, u32), u32);
type U32x5 = ((U32x3, u32), u32);
type U32x6 = (U32x5, u32);
type U32x10 = ((((U32x6, u32), u32), u32), u32);

trait ToArray {
    type Output;
    fn to_array(self) -> Self::Output;
}

impl ToArray for U32x3 {
    type Output = [u32; 3];
    fn to_array(self) -> Self::Output {
        let ((x0, x1), x2) = self;
        [x0, x1, x2]
    }
}

impl ToArray for U32x5 {
    type Output = [u32; 5];
    fn to_array(self) -> Self::Output {
        let ((((x0, x1), x2), x3), x4) = self;
        [x0, x1, x2, x3, x4]
    }
}

impl ToArray for U32x6 {
    type Output = [u32; 6];
    fn to_array(self) -> Self::Output {
        let (((((x0, x1), x2), x3), x4), x5) = self;
        [x0, x1, x2, x3, x4, x5]
    }
}

impl ToArray for U32x10 {
    type Output = [u32; 10];
    fn to_array(self) -> Self::Output {
        let (((((((((x0, x1), x2), x3), x4), x5), x6), x7), x8), x9) = self;
        [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9]
    }
}

trait Crt {
    type Input;
    type Output;
    fn crt(i: Self::Input) -> Self::Output;
}

impl Crt for CrtU64 {
    type Input = U32x3;
    type Output = u64;
    fn crt(xs: Self::Input) -> u64 {
        let [x0, x1, x2] = xs.to_array();
        [
            (x0 as u64, MOD0 as u64),
            (x1 as u64, MOD1 as u64),
            (x2 as u64, MOD2 as u64),
        ]
        .crt_wrapping()
    }
}

impl Crt for CrtWrappingU64 {
    type Input = U32x6;
    type Output = u64;
    fn crt(xs: Self::Input) -> u64 {
        let [x0, x1, x2, x3, x4, x5] = xs.to_array();
        [
            (x0 as u64, MOD0 as u64),
            (x1 as u64, MOD1 as u64),
            (x2 as u64, MOD2 as u64),
            (x3 as u64, MOD3 as u64),
            (x4 as u64, MOD4 as u64),
            (x5 as u64, MOD5 as u64),
        ]
        .crt_wrapping()
    }
}

impl Crt for CrtU128 {
    type Input = U32x5;
    type Output = u128;
    fn crt(xs: Self::Input) -> u128 {
        let [x0, x1, x2, x3, x4] = xs.to_array();
        [
            (x0 as u128, MOD0 as u128),
            (x1 as u128, MOD1 as u128),
            (x2 as u128, MOD2 as u128),
            (x3 as u128, MOD3 as u128),
            (x4 as u128, MOD4 as u128),
        ]
        .crt_wrapping()
    }
}

impl Crt for CrtWrappingU128 {
    type Input = U32x10;
    type Output = u128;
    fn crt(xs: Self::Input) -> u128 {
        let [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9] = xs.to_array();
        [
            (x0 as u128, MOD0 as u128),
            (x1 as u128, MOD1 as u128),
            (x2 as u128, MOD2 as u128),
            (x3 as u128, MOD3 as u128),
            (x4 as u128, MOD4 as u128),
            (x5 as u128, MOD5 as u128),
            (x6 as u128, MOD6 as u128),
            (x7 as u128, MOD7 as u128),
            (x8 as u128, MOD8 as u128),
            (x9 as u128, MOD9 as u128),
        ]
        .crt_wrapping()
    }
}

trait CrtMod {
    type Input;
    type Output;
    fn crt_mod(self, i: Self::Input) -> Self::Output;
}

impl CrtU32Mod {
    fn new(m: u32) -> Self { Self(m) }
}

impl CrtU64Mod {
    fn new(m: u64) -> Self { Self(m) }
}

impl CrtU128Mod {
    fn new(m: u128) -> Self { Self(m) }
}

impl CrtMod for CrtU32Mod {
    type Input = U32x3;
    type Output = u32;
    fn crt_mod(self, xs: Self::Input) -> Self::Output {
        let [x0, x1, x2] = xs.to_array();
        [
            (x0 as u64, MOD0 as u64),
            (x1 as u64, MOD1 as u64),
            (x2 as u64, MOD2 as u64),
        ]
        .crt_mod(self.0 as u64) as u32
    }
}

impl CrtMod for CrtU64Mod {
    type Input = U32x6;
    type Output = u64;
    fn crt_mod(self, xs: Self::Input) -> Self::Output {
        let [x0, x1, x2, x3, x4, x5] = xs.to_array();
        [
            (x0 as u64, MOD0 as u64),
            (x1 as u64, MOD1 as u64),
            (x2 as u64, MOD2 as u64),
            (x3 as u64, MOD3 as u64),
            (x4 as u64, MOD4 as u64),
            (x5 as u64, MOD5 as u64),
        ]
        .crt_mod(self.0)
    }
}

impl CrtMod for CrtU128Mod {
    type Input = U32x10;
    type Output = u128;
    fn crt_mod(self, xs: Self::Input) -> Self::Output {
        let [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9] = xs.to_array();
        [
            (x0 as u128, MOD0 as u128),
            (x1 as u128, MOD1 as u128),
            (x2 as u128, MOD2 as u128),
            (x3 as u128, MOD3 as u128),
            (x4 as u128, MOD4 as u128),
            (x5 as u128, MOD5 as u128),
            (x6 as u128, MOD6 as u128),
            (x7 as u128, MOD7 as u128),
            (x8 as u128, MOD8 as u128),
            (x9 as u128, MOD9 as u128),
        ]
        .crt_mod(self.0)
    }
}

macro_rules! impl_convolve {
    ( $( ($fn:ident, $ty:ty, $crt:path, [$mod1:ty, $( $mod:ty ),*]), )* ) => { $(
        pub fn $fn(a: &[$ty], b: &[$ty]) -> Vec<$ty> {
            if a.is_empty() || b.is_empty() {
                return vec![];
            }
            let n = a.len();
            let m = b.len();
            assert!(n + m - 1 <= 1_usize << <$mod1>::EXP);
            $( assert!(n + m - 1 <= 1_usize << <$mod>::EXP) );*;
            convolve_from::<$mod1, _>(&a, &b)
                .into_iter()
                $( .zip(convolve_from::<$mod, _>(&a, &b)) )*
                .map($crt)
                .collect()
        }
    )* }
}

macro_rules! impl_convolve_mod {
    ( $( ($fn:ident, $ty:ty, $crt:ident, [$mod1:ty, $( $mod:ty ),*]), )* ) => { $(
        pub fn $fn(a: &[$ty], b: &[$ty], mm: $ty) -> Vec<$ty> {
            if a.is_empty() || b.is_empty() {
                return vec![];
            }
            let n = a.len();
            let m = b.len();
            assert!(n + m - 1 <= 1_usize << <$mod1>::EXP);
            $( assert!(n + m - 1 <= 1_usize << <$mod>::EXP) );*;
            let crt = $crt::new(mm);
            convolve_from::<$mod1, _>(&a, &b)
                .into_iter()
                $( .zip(convolve_from::<$mod, _>(&a, &b)) )*
                .map(|x| crt.crt_mod(x))
                .collect()
        }
    )* }
}

impl_convolve! {
    (convolve_u64, u64, CrtU64::crt, [Mod0, Mod1, Mod2]),
    (convolve_u128, u128, CrtU128::crt, [Mod0, Mod1, Mod2, Mod3, Mod4]),
    (convolve_wrapping_u64, u64, CrtWrappingU64::crt, [Mod0, Mod1, Mod2, Mod3, Mod4, Mod5]),
    (convolve_wrapping_u128, u128, CrtWrappingU128::crt, [Mod0, Mod1, Mod2, Mod3, Mod4, Mod5, Mod6, Mod7, Mod8, Mod9]),
}

impl_convolve_mod! {
    (convolve_u32_mod, u32, CrtU32Mod, [Mod0, Mod1, Mod2]),
    (convolve_u64_mod, u64, CrtU64Mod, [Mod0, Mod1, Mod2, Mod3, Mod4, Mod5]),
    (convolve_u128_mod, u128, CrtU128Mod, [Mod0, Mod1, Mod2, Mod3, Mod4, Mod5, Mod6, Mod7, Mod8, Mod9]),
}

fn ceil_pow2(n: u32) -> u32 { 32 - n.saturating_sub(1).leading_zeros() }

#[test]
fn sanity_check() {
    type Mi = modint::ModInt998244353;

    let a: Vec<_> = [0, 1, 2, 3, 4].iter().map(|&x| Mi::new(x)).collect();
    let b: Vec<_> = [0, 1, 2, 4, 8].iter().map(|&x| Mi::new(x)).collect();
    let c: Vec<_> = convolve_fft(a, b).iter().map(|x| x.get()).collect();

    assert_eq!(c, [0, 0, 1, 4, 11, 26, 36, 40, 32]);
}

#[test]
fn proot() {
    assert_eq!(Mod45e24p1::PRIMITIVE_ROOT, 11);
    assert_eq!(Mod73e24p1::PRIMITIVE_ROOT, 3);
    assert_eq!(Mod127e24p1::PRIMITIVE_ROOT, 3);
    assert_eq!(Mod5e25p1::PRIMITIVE_ROOT, 3);
    assert_eq!(Mod33e25p1::PRIMITIVE_ROOT, 10);
    assert_eq!(Mod51e25p1::PRIMITIVE_ROOT, 29);
    assert_eq!(Mod63e25p1::PRIMITIVE_ROOT, 5);
    assert_eq!(Mod7e26p1::PRIMITIVE_ROOT, 3);
    assert_eq!(Mod27e26p1::PRIMITIVE_ROOT, 13);
    assert_eq!(Mod15e27p1::PRIMITIVE_ROOT, 31);
}

#[test]
fn large() {
    let max32 = u32::MAX as u64;
    assert_eq!(convolve_u64(&[max32], &[max32]), [max32 * max32]);

    let max64 = u64::MAX as u128;
    assert_eq!(convolve_u128(&[max64], &[max64]), [max64 * max64]);
}

#[test]
fn long_wrapping_u64() {
    let max32 = u32::MAX as u64;
    let n = 1 << 24;
    let long32 = vec![max32; n];
    let a = convolve_wrapping_u64(&long32, &long32);
    for i in 0..n {
        assert_eq!(a[i], a[n + n - 2 - i]);
        assert_eq!(a[i], (max32 * max32).wrapping_mul(i as u64 + 1));
    }
}

#[test]
fn long_wrapping_u128() {
    let max64 = u64::MAX as u128;
    let n = 1 << 23;
    let long64 = vec![max64; n];
    let a = convolve_wrapping_u128(&long64, &long64);
    for i in 0..n {
        assert_eq!(a[i], a[n + n - 2 - i]);
        assert_eq!(a[i], (max64 * max64).wrapping_mul(i as u128 + 1));
    }
}

#[test]
fn long_u32_mod() {
    let max32 = u32::MAX;
    let n = 1 << 24;
    let p = 998244353;
    let long32 = vec![max32; n];
    let a = convolve_u32_mod(&long32, &long32, p as u32);
    for i in 0..n {
        assert_eq!(a[i], a[n + n - 2 - i]);
        let expected = (max32 as u64 % p).pow(2) % p * (i as u64 + 1) % p;
        assert_eq!(a[i], expected as u32);
    }
}

#[test]
fn long_u64_mod() {
    let max32 = u32::MAX as u64;
    let n = 1 << 24;
    let p = 998244353;
    let long32 = vec![max32; n];
    let a = convolve_u64_mod(&long32, &long32, p);
    for i in 0..n {
        assert_eq!(a[i], a[n + n - 2 - i]);
        assert_eq!(a[i], (max32 * max32 % p) * (i as u64 + 1) % p);
    }
}

#[test]
fn long_u128_mod() {
    let max64 = u64::MAX as u128;
    let n = 1 << 23;
    let p = 998244353;
    let long64 = vec![max64; n];
    let a = convolve_u128_mod(&long64, &long64, p);
    for i in 0..n {
        assert_eq!(a[i], a[n + n - 2 - i]);
        assert_eq!(a[i], (max64 * max64 % p) * (i as u128 + 1) % p);
    }
}
