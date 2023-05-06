use std::cell::RefCell;
use std::thread::LocalKey;

use modint::{Mod998244353, ModIntBase, Modulus, StaticModInt};

pub struct ButterflyCache<M: NttFriendly> {
    root: Vec<StaticModInt<M>>,
    iroot: Vec<StaticModInt<M>>,
    rate2: Vec<StaticModInt<M>>,
    irate2: Vec<StaticModInt<M>>,
    rate3: Vec<StaticModInt<M>>,
    irate3: Vec<StaticModInt<M>>,
}

pub trait NttFriendly: Modulus {
    const PRIMITIVE_ROOT: u32;
    // ODD << EXP | 1 == MOD
    const EXP: u32 = (Self::VALUE - 1).trailing_zeros();
    const ODD: u32 = Self::VALUE >> Self::EXP;

    fn butterfly_cache()
    -> &'static LocalKey<RefCell<Option<ButterflyCache<Self>>>>;
}

impl NttFriendly for Mod998244353 {
    const PRIMITIVE_ROOT: u32 = 3;
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

fn butterfly<M: NttFriendly>(a: &mut [StaticModInt<M>]) {
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

fn butterfly_inv<M: NttFriendly>(a: &mut [StaticModInt<M>]) {
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
    ( $( ($mod:ident, $val:expr, $modint:ident, $g:literal), )* ) => { $(
        #[derive(Clone, Copy, Eq, PartialEq)]
        struct $mod;
        impl Modulus for $mod {
            const VALUE: u32 = $val;
        }
        type $modint = StaticModInt<$mod>;
        impl NttFriendly for $mod {
            const PRIMITIVE_ROOT: u32 = $g;
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
    (Mod45e24p1, 45 << 24 | 1, ModInt754974721, 11),
    (Mod5e25p1, 5 << 25 | 1, ModInt167772161, 3),
    (Mod7e26p1, 7 << 26 | 1, ModInt469762049, 3),
}

fn convolve_from_u64<M: NttFriendly>(a: &[u64], b: &[u64]) -> Vec<u64> {
    let a: Vec<_> = a.iter().map(|&ai| StaticModInt::new(ai)).collect();
    let b: Vec<_> = b.iter().map(|&bi| StaticModInt::new(bi)).collect();
    convolve(a, b)
        .into_iter()
        .map(|x: StaticModInt<M>| x.get() as u64)
        .collect()
}

pub fn convolve_u64(a: &[u64], b: &[u64]) -> Vec<u64> {
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

    let i1 = ModInt754974721::new(m2m3).recip().get() as u64;
    let i2 = ModInt167772161::new(m1m3).recip().get() as u64;
    let i3 = ModInt469762049::new(m1m2).recip().get() as u64;

    let max_bit = 24;
    assert_eq!(mod1 % (1 << max_bit), 1);
    assert_eq!(mod2 % (1 << max_bit), 1);
    assert_eq!(mod3 % (1 << max_bit), 1);
    assert!(n + m - 1 <= (1 << max_bit));

    let c1 = convolve_from_u64::<Mod45e24p1>(&a, &b);
    let c2 = convolve_from_u64::<Mod5e25p1>(&a, &b);
    let c3 = convolve_from_u64::<Mod7e26p1>(&a, &b);

    c1.into_iter()
        .zip(c2)
        .zip(c3)
        .map(|((c1i, c2i), c3i)| {
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

fn ceil_pow2(n: u32) -> u32 { 32 - n.saturating_sub(1).leading_zeros() }

#[test]
fn sanity_check() {
    type Mi = modint::ModInt998244353;

    let a: Vec<_> = [0, 1, 2, 3, 4].iter().map(|&x| Mi::new(x)).collect();
    let b: Vec<_> = [0, 1, 2, 4, 8].iter().map(|&x| Mi::new(x)).collect();
    let c: Vec<_> = convolve_fft(a, b).iter().map(|x| x.get()).collect();

    assert_eq!(c, [0, 0, 1, 4, 11, 26, 36, 40, 32]);
}
