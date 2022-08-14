#[derive(Clone, Copy)]
pub struct PdepPextMaskU8([u8; 3], u8);
#[derive(Clone, Copy)]
pub struct PdepPextMaskU16([u16; 4], u16);
#[derive(Clone, Copy)]
pub struct PdepPextMaskU32([u32; 5], u32);
#[derive(Clone, Copy)]
pub struct PdepPextMaskU64([u64; 6], u64);
#[derive(Clone, Copy)]
pub struct PdepPextMaskU128([u128; 7], u128);

pub trait Pext<R> {
    fn pext(self, mask: R) -> Self;
}

macro_rules! impl_pext {
    ( ($maskty:ident, $basety:ident, $lg:literal) ) => {
        impl Pext<$basety> for $basety {
            fn pext(self, mut m: $basety) -> $basety {
                let mut x = self & m;
                let mut mk = !m << 1;
                for i in 0..$lg {
                    let mut mp = mk ^ (mk << 1);
                    for j in 1..$lg {
                        mp ^= mp << (1 << j);
                    }
                    let mv = mp & m;
                    m = m ^ mv | (mv >> (1 << i));
                    let t = x & mv;
                    x = (x ^ t) | (t >> (1 << i));
                    mk &= !mp;
                }
                x
            }
        }
        impl Pext<$maskty> for $basety {
            fn pext(self, mask: $maskty) -> $basety {
                let mut x = self & mask.1;
                for i in 0..$lg {
                    let mv = mask.0[i];
                    let t = x & mv;
                    x = (x ^ t) | (t >> (1 << i));
                }
                x
            }
        }
    };
    ( $( ( $($tt:tt)* ), )* ) => { $( impl_pext!( ( $($tt)* ) ); )* };
}

impl_pext! {
    (PdepPextMaskU8, u8, 3),
    (PdepPextMaskU16, u16, 4),
    (PdepPextMaskU32, u32, 5),
    (PdepPextMaskU64, u64, 6),
    (PdepPextMaskU128, u128, 7),
}

pub trait Pdep<R> {
    fn pdep(self, mask: R) -> Self;
}

macro_rules! impl_pdep {
    ( ($maskty:ident, $basety:ident, $lg:literal) ) => {
        impl Pdep<$basety> for $basety {
            fn pdep(self, m: $basety) -> $basety {
                self.pdep(<$maskty>::new(m))
            }
        }
        impl Pdep<$maskty> for $basety {
            fn pdep(self, mask: $maskty) -> $basety {
                let mut x = self;
                for i in (0..$lg).rev() {
                    let mv = mask.0[i];
                    let t = x << (1 << i);
                    x = (x & !mv) | (t & mv);
                }
                x & mask.1
            }
        }
    };
    ( $( ( $($tt:tt)* ), )* ) => { $( impl_pdep!( ( $($tt)* ) ); )* };
}

impl_pdep! {
    (PdepPextMaskU8, u8, 3),
    (PdepPextMaskU16, u16, 4),
    (PdepPextMaskU32, u32, 5),
    (PdepPextMaskU64, u64, 6),
    (PdepPextMaskU128, u128, 7),
}

macro_rules! pext_loop {
    ( $mk:ident, $m:ident, $sh:expr ) => {{
        let mp = Self::mp($mk);
        let mv = mp & $m;
        $m = $m ^ mv | (mv >> (1 << $sh));
        $mk &= !mp;
        mv
    }};
}

macro_rules! impl_pdep_pext_mask {
    ( ($maskty:ident, $basety:ident, [$($i:literal),*], $lg:literal) ) => {
        impl $maskty {
            pub const fn new(mut m: $basety) -> Self {
                let m0 = m;
                let mut res = [0; $lg];
                let mut mk = !m << 1;
                $( res[$i] = pext_loop!(mk, m, $i) );*;
                res[$lg - 1] = Self::mp(mk) & m;
                Self(res, m0)
            }
            const fn mp(mk: $basety) -> $basety {
                let mut mp = mk ^ (mk << 1);
                $( mp ^= mp << (1 << (1 + $i)) );*;
                mp
            }
            pub const fn get(self) -> $basety { self.1 }
        }
    };
    ( $( ( $($tt:tt)* ), )* ) => { $( impl_pdep_pext_mask!( ( $($tt)* ) ); )* };
}

impl_pdep_pext_mask! {
    (PdepPextMaskU8, u8, [0, 1], 3),
    (PdepPextMaskU16, u16, [0, 1, 2], 4),
    (PdepPextMaskU32, u32, [0, 1, 2, 3], 5),
    (PdepPextMaskU64, u64, [0, 1, 2, 3, 4], 6),
    (PdepPextMaskU128, u128, [0, 1, 2, 3, 4, 5], 7),
}

#[test]
fn test() {
    let x = 0b_0101_0111_0000_1001_1110_1010_0000_0010_u32;
    let m = 0b_0100_1001_1001_1010_0100_0100_0101_0100_u32;
    //          1   0  1 0  0 1 0   1    0    0 0  0
    let ext = 0b_1010_0101_0000;

    assert_eq!(x.pext(m), ext);
    assert_eq!(x.pext(PdepPextMaskU32::new(m)), ext);
    assert_eq!(ext.pdep(m), x & m);
}
