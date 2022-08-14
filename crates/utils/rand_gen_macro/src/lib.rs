//! 乱数生成マクロ。

#[macro_export]
macro_rules! rand_gen_builder {
    ( @gen { [ $($cur:tt)* ] } ) => {
        $crate::rand_gen_builder!(@vec @gen {} @rest $($cur)*)
    };
    ( @gen { [ $($cur:tt)* ] where { $($f:ident $(= $a:expr)?),* } } ) => {
        $crate::rand_gen_builder!(@vec @gen {} @rest $($cur)*)
            .options() $(.$f($($a)?))*
    };
    ( @gen { [ $($cur:tt)* ] where { $($f:ident $(= $a:expr)?),*, } } ) => {
        $crate::rand_gen_builder!(@vec @gen {} @rest $($cur)*)
            .options() $(.$f($($a)?))*
    };
    ( @vec @gen { $($x:tt)* } @rest ; $($rest:tt)* ) => {
        $crate::rand_gen_builder!(@vec @gen { $($x)* } @len { $($rest)* })
    };
    ( @vec @gen { $($cur:tt)* } @rest $tt:tt $($rest:tt)* ) => {
        $crate::rand_gen_builder!(@vec @gen { $($cur)* $tt } @rest $($rest)*)
    };
    ( @vec @gen { $($x:tt)* } @len { $($len:tt)* }) => {
        VecMarker::new($crate::rand_gen_builder!(@gen { $($x)* }), $($len)*)
    };

    ( @gen { $($x:tt)* } ) => {
        $crate::rand_gen_builder!(@gen {} @rest $($x)*)
    };
    ( @gen { $($cur:tt)* } @rest where { $($where:tt)* } ) => {
        $crate::rand_gen_builder!(@gen { $($cur)* } @where { $($where)* } @rest)
    };
    ( @gen { $($cur:tt)* } @rest $tt:tt $($rest:tt)* ) => {
        $crate::rand_gen_builder!(@gen { $($cur)* $tt } @rest $($rest)*)
    };
    ( @gen { $($cur:tt)* } @where { $($f:ident $(= $a:expr)?),* } @rest ) => {
        $crate::rand_gen_builder!(@gen { $($cur)* } @rest)
            .options() $(.$f($($a)?))*
    };
    ( @gen { $($cur:tt)* } @where { $($f:ident $(= $a:expr)?),*, } @rest ) => {
        $crate::rand_gen_builder!(@gen { $($cur)* } @rest)
            .options() $(.$f($($a)?))*
    };
    ( @gen { $($x:tt)* } @rest ) => {
        ($($x)*)
    };
}

/// 乱数生成マクロ。
///
/// # Notes
/// 作りかけなのでいろいろ足りない。
///
/// # Examples
/// ```
/// use rand::SeedableRng;
/// use rand_chacha::ChaCha20Rng;
///
/// use nekolib::rand_gen;
/// use nekolib::utils::ascii::*;
/// use nekolib::utils::rand_gen_macro::*;
///
/// rand_gen! {
///     rng: ChaCha20Rng;
///
///     n in 1_usize..=10;
///     a in [1_i64..=100; n];
///     s in AsciiString(16) where {
///         distribution = &[
///             (ASCII_LOWERCASE, 10),
///             (ASCII_UPPERCASE, 10),
///             (ASCII_DIGIT, 6),
///             (charset(b"~!@#$%+?|()^*_-=[]{};:,./"), 5),
///         ],
///     };
/// }
///
/// assert_eq!(a.len(), n);
/// assert!(a.iter().all(|ai| (1..=100).contains(ai)));
/// assert_eq!(s.len(), 16);
/// // Possible value of `s`: `"3e)xIjos2^M/XI1T"`, `"X52dhjDk%i6)p1F9"`
/// ```
///
/// 以下のような出力が得られるため、これを用いて再現することができる。
///
/// ````text
/// To reproduce:
///
/// ```
/// rand_gen! {
///     rng = ChaCha20Rng::from_seed([250, 120, 31, 164, 15, 176, 41, 144, 61, 59, 224, 119, 135, 238, 14, 193, 149, 124, 228, 39, 107, 208, 243, 180, 7, 177, 21, 88, 19, 5, 225, 3]);
///     // ...
/// }
/// ```
/// ````
///
/// ```
/// use rand::SeedableRng;
/// use rand_chacha::ChaCha20Rng;
///
/// use nekolib::rand_gen;
/// use nekolib::utils::ascii::*;
/// use nekolib::utils::rand_gen_macro::*;
///
/// rand_gen! {
///     rng = ChaCha20Rng::from_seed([250, 120, 31, 164, 15, 176, 41, 144, 61, 59, 224, 119, 135, 238, 14, 193, 149, 124, 228, 39, 107, 208, 243, 180, 7, 177, 21, 88, 19, 5, 225, 3]);
///
///     n in 1_usize..=10;
///     a in [1_i64..=100; n];
///     s in AsciiString(16) where {
///         distribution = &[
///             (ASCII_LOWERCASE, 10),
///             (ASCII_UPPERCASE, 10),
///             (ASCII_DIGIT, 6),
///             (charset(b"~!@#$%+?|()^*_-=[]{};:,./"), 5),
///         ],
///     };
/// }
///
/// assert_eq!(a, [32, 86, 41, 68, 66, 46, 56, 82, 40, 1]);
/// assert_eq!(s, "X52dhjDk%i6)p1F9");
/// ```
#[macro_export]
macro_rules! rand_gen {
    ( @seed $seed:ident { $a:ident in $($r:tt)* } @rest ) => {
        let $a = $seed.generate($crate::rand_gen_builder!(@gen { $($r)* }));
    };
    ( @seed $seed:ident { $a:ident in $($r:tt)* } @rest ; $($rest:tt)* ) => {
        rand_gen!(@seed $seed { $a in $($r)* } @rest);
        rand_gen!(@seed $seed {} @rest $($rest)*);
    };
    ( @seed $seed:ident { $($cur:tt)* } @rest $tt:tt $($rest:tt)* ) => {
        rand_gen!(@seed $seed { $($cur)* $tt } @rest $($rest)* );
    };
    ( @seed $seed:ident {} @rest ) => {};
    ( $seed:ident: $ty:ty; $($rest:tt)* ) => {
        // let mut $seed = <$ty as RandomWordGenerator>::auto_init();
        let mut $seed = <$ty>::from_entropy();
        eprintln!(r#"
To reproduce:

```
rand_gen! {{
    {} = {};
    // ...
}}
```
"#, stringify!($seed), $seed.inspect());
        rand_gen!(@seed $seed {} @rest $($rest)*);
    };
    ( $seed:ident = $s:expr; $($rest:tt)* ) => {
        let mut $seed = $s;
        // $seed.inspect(stringify!($seed));
        rand_gen!(@seed $seed {} @rest $($rest)*);
    };
}

// .options() いらないかも
pub trait GenOptions {
    type OptionType;
    fn options(self) -> Self::OptionType;
}

// ---
// ```
// pub trait RandomGenerator<Input> {
//     type Output;
//     fn generate(&mut self) -> Self::Output;
// }
// ```
// みたいなのを作って、impl RandChaCha とかする？
// Input = RangeInclusive<i64>, Output = i64 とかを想定している。
//
// 今だと impl RandGen for Input して rand_gen の引数に &mut RandChaCha とかの
// を渡すようにしているのか、むー。でもこれが適当な impl trait なのでつらいんだね
//
// なんかできてそう。
// あとは、過去のを消しつつ、options をよしなに解決すればよさそう。

#[derive(Clone, Copy)]
pub struct AsciiString(pub usize);

impl GenOptions for AsciiString {
    type OptionType = Self;
    fn options(self) -> Self { self }
}

#[derive(Clone, Copy)]
pub struct AsciiStringOfCharset(AsciiGen, usize);

#[derive(Clone)]
pub struct AsciiStringOfDistribution(BTreeMap<u32, AsciiGen>, u32, usize);

impl AsciiString {
    pub fn charset(self, cs: u128) -> AsciiStringOfCharset {
        AsciiStringOfCharset(AsciiGen::new(cs), self.0)
    }
    pub fn distribution(self, d: &[(u128, u32)]) -> AsciiStringOfDistribution {
        let mut map = BTreeMap::new();
        let mut acc = 0;
        for &(mask, p) in d.iter().filter(|&&(_, p)| p > 0) {
            acc += p;
            map.insert(acc, AsciiGen::new(mask));
        }
        AsciiStringOfDistribution(map, acc, self.0)
    }
}

impl RandomGenerator<AsciiStringOfCharset> for ChaCha20Rng {
    type Output = String;
    fn generate(&mut self, subject: AsciiStringOfCharset) -> String {
        let AsciiStringOfCharset(gen, len) = subject;
        if len == 0 {
            "".to_owned()
        } else {
            (0..len).map(|_| self.generate(gen)).collect()
        }
    }
}

impl RandomGenerator<AsciiStringOfDistribution> for ChaCha20Rng {
    type Output = String;
    fn generate(&mut self, subject: AsciiStringOfDistribution) -> String {
        let AsciiStringOfDistribution(ref gen_map, acc, len) = subject;
        let large = Uniform::from(0..acc);
        (0..len)
            .map(|_| {
                let c = large.sample(self);
                let small = gen_map.range(c..).next().unwrap().1;
                self.generate(*small)
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
pub struct Ascii;

impl GenOptions for Ascii {
    type OptionType = Self;
    fn options(self) -> Ascii { self }
}

impl Ascii {
    pub fn charset(self, cs: u128) -> AsciiGen { AsciiGen::new(cs) }
}

#[derive(Clone, Copy)]
pub struct AsciiGen(PdepPextMaskU128, u32);

impl AsciiGen {
    pub fn new(mask: u128) -> Self {
        Self(PdepPextMaskU128::new(mask), mask.count_ones())
    }
}

impl RandomGenerator<AsciiGen> for ChaCha20Rng {
    type Output = char;
    fn generate(&mut self, subject: AsciiGen) -> char {
        let AsciiGen(mask, pop) = subject;
        assert_ne!(pop, 0, "empty charset");
        let c = Uniform::from(0..pop).sample(self);
        (1 << c).pdep(mask).trailing_zeros() as u8 as char
    }
}

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Range, RangeInclusive};

use rand::distributions::{Distribution, Uniform};
use rand_chacha::ChaCha20Rng;

use bitop::{Pdep, PdepPextMaskU128};

pub trait SeedableRngInspect {
    fn inspect(&self) -> String;
}

impl SeedableRngInspect for ChaCha20Rng {
    fn inspect(&self) -> String {
        format!("ChaCha20Rng::from_seed({:?})", self.get_seed())
    }
}

pub trait RandomGenerator<Input> {
    type Output;
    fn generate(&mut self, subject: Input) -> Self::Output;
}

macro_rules! impl_range {
    ( $($t:ty)* ) => { $(
        impl RandomGenerator<RangeInclusive<$t>> for ChaCha20Rng {
            type Output = $t;
            fn generate(&mut self, s: RangeInclusive<$t>) -> $t {
                let between = Uniform::from(s);
                between.sample(self)
            }
        }
        impl RandomGenerator<Range<$t>> for ChaCha20Rng {
            type Output = $t;
            fn generate(&mut self, s: Range<$t>) -> $t {
                let between = Uniform::from(s);
                between.sample(self)
            }
        }
    )* }
}

impl_range! { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize }

pub struct VecMarker<T> {
    inner: T,
    len: usize,
}

impl<T> VecMarker<T> {
    pub fn new(inner: T, len: usize) -> Self { Self { inner, len } }
}

// sorted は Ord、distinct は Eq が必要になるけど、そもそも
// 範囲で取ってランダムに生成とか言っている時点でそれらは仮定していそう。
pub struct VecOptionsMarker<T> {
    inner: T,
    len: usize,
    sorted: bool,
    distinct: bool,
}

impl<T> GenOptions for VecMarker<T> {
    type OptionType = VecOptionsMarker<T>;
    fn options(self) -> VecOptionsMarker<T> {
        let Self { inner, len } = self;
        VecOptionsMarker { inner, len, sorted: false, distinct: false }
    }
}

impl<T> VecOptionsMarker<T> {
    pub fn sorted(mut self) -> Self {
        self.sorted = true;
        self
    }
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }
}

impl RandomGenerator<VecMarker<RangeInclusive<i64>>> for ChaCha20Rng {
    type Output = Vec<i64>;
    fn generate(
        &mut self,
        subject: VecMarker<RangeInclusive<i64>>,
    ) -> Vec<i64> {
        let VecMarker { inner, len } = subject;
        let between = Uniform::from(inner);
        (0..len).map(|_| between.sample(self)).collect()
    }
}

impl RandomGenerator<VecOptionsMarker<RangeInclusive<i64>>> for ChaCha20Rng {
    type Output = Vec<i64>;
    fn generate(
        &mut self,
        subject: VecOptionsMarker<RangeInclusive<i64>>,
    ) -> Vec<i64> {
        let VecOptionsMarker { inner, len, sorted, distinct } = subject;

        if len == 0 {
            return vec![];
        }

        let start = *inner.start();
        let end = *inner.end();
        if start > end {
            panic!("Emptyset");
        }
        if distinct {
            let u_len = end - start;
            // 全域だったらオーバーフローするので +1 しないでおく。
            // len == 0 は上で処理しているので、len-1 はオーバーフローしない。
            // 符号つきだから u_len の時点でオーバーフローしうるじゃんね。
            // 全域でなければ Range の方を使うようにする？ 全域なら別で考える？
            // 嫌だけど。

            // if end - start + 1 < len {}
            if u_len == 0 && len == 1 {
                return vec![start];
            }
            if (u_len as u128) < len as u128 - 1 {
                panic!("by pigeonhole principle, it is infeasible");
            }

            // (end - start + 1, len)
            // (_, 0) => []
            // (1, 1) => [start]
            // (x, y) if x < y => panic!()

            let mut res = vec![];
            // k >~ n/log(n) くらいなら dense だと思っていい？
            // ゼロ割りは嫌なので k log(n) >~ n。
            let lg_len = len.next_power_of_two().trailing_zeros() as i64;
            let sparse = u_len * lg_len < len as i64;

            if sparse {
                let between = Uniform::from(inner);
                let mut seen = BTreeSet::new();
                while res.len() < len {
                    // 失敗しまくったら下のやり方に fallback した方がいい？
                    let cur = between.sample(self);
                    if seen.insert(cur) {
                        res.push(cur);
                    }
                }
            } else {
                let mut pool: Vec<_> = (start..=end).collect();
                for _ in 0..len {
                    let u = Uniform::from(0..pool.len());
                    let i = u.sample(self);
                    res.push(pool.swap_remove(i));
                }
            }

            if sorted {
                res.sort_unstable();
            }
            return res;
        }

        if !sorted {
            return self.generate(VecMarker { inner, len });
        }

        // todo うまくやる
        // distinct のオプションがあるとやりやすいかも
        // 関数が同じだから再帰でやるかどうしようか
        // [start..=end + len - 1; len] where { distinct }
        // を作り、ソートして、a[i] -= i して返す。
        //
        // distinct に作るパートが難しくて、sparse か dense かで分ける？
        // infeasible なら panic!() で。

        // let mut tmp = self.generate(VecMarker { inner, len });
        // tmp.sort_unstable();
        // tmp

        let start = *inner.start();
        let end = inner.end() + len as i64 - 1;
        let mut res = self.generate(VecOptionsMarker {
            inner: start..=end,
            len,
            sorted: true,
            distinct: true,
        });
        for i in 0..len {
            res[i] -= i as i64;
        }
        res
    }
}
