pub trait MillerRabin {
    fn is_prime(self) -> bool;
}

impl MillerRabin for u8 {
    fn is_prime(self) -> bool {
        let x = self;
        if x == 2 || x == 3 || x == 5 || x == 7 || x == 11 || x == 13 {
            return true;
        }
        x > 1
            && x % 2 > 0
            && x % 3 > 0
            && x % 5 > 0
            && x % 7 > 0
            && x % 11 > 0
            && x % 13 > 0
    }
}

impl MillerRabin for u16 {
    fn is_prime(self) -> bool { (self as u32).is_prime() }
}

impl MillerRabin for u32 {
    fn is_prime(self) -> bool {
        let x = self;
        if x == 2 || x == 3 || x == 5 || x == 7 {
            return true;
        }
        if x % 2 == 0 || x % 3 == 0 || x % 5 == 0 || x % 7 == 0 {
            return false;
        }
        if x < 121 {
            return x > 1;
        }
        let h = x as u64;
        let h = ((h >> 16) ^ h).wrapping_mul(0x45d9f3b);
        let h = ((h >> 16) ^ h).wrapping_mul(0x45d9f3b);
        let h = ((h >> 16) ^ h) & 255;
        is_sprp_32(x, BASES[h as usize] as u32)
    }
}

impl MillerRabin for u64 {
    fn is_prime(self) -> bool {
        let x = self;
        if x == 2 || x == 3 || x == 5 || x == 7 {
            return true;
        }
        if x % 2 == 0 || x % 3 == 0 || x % 5 == 0 || x % 7 == 0 {
            return false;
        }
        if x < 121 {
            return x > 1;
        }

        [2, 325, 9375, 28178, 450775, 9780504, 1795265022]
            .iter()
            .all(|&b| b % x == 0 || is_sprp_64(x, b % x))
    }
}

// Forišek, Michal, and Jakub Jancina.
// "Fast Primality Testing for Integers That Fit into a Machine Word." (2015).
fn is_sprp_32(n: u32, a: u32) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = n >> s;
    let mut cur = {
        let mut cur = 1;
        let mut pow = d;
        let mut a = a;
        while pow > 0 {
            if pow & 1 != 0 {
                cur = (cur as u64 * a as u64 % n as u64) as u32;
            }
            a = ((a as u64).pow(2) % n as u64) as u32;
            pow >>= 1;
        }
        cur
    };
    if cur == 1 {
        return true;
    }
    for _ in 0..s {
        if cur == n - 1 {
            return true;
        }
        cur = ((cur as u64).pow(2) % n as u64) as u32;
    }
    false
}

#[rustfmt::skip]
const BASES: [u16; 256] = [
    0x3ce7, 0x07e2, 0x00a6, 0x1d05, 0x1f80, 0x3ead, 0x2907, 0x112f,
    0x079d, 0x050f, 0x0ad8, 0x0e24, 0x0230, 0x0c38, 0x145c, 0x0a61,
    0x08fc, 0x07e5, 0x122c, 0x05bf, 0x2478, 0x0fb2, 0x095e, 0x4fee,
    0x2825, 0x1f5c, 0x08a5, 0x184b, 0x026c, 0x0eb3, 0x12f4, 0x1394,
    0x0c71, 0x0535, 0x1853, 0x14b2, 0x0432, 0x0957, 0x13f9, 0x1b95,
    0x0323, 0x04f5, 0x0f23, 0x01a6, 0x02ef, 0x0244, 0x1279, 0x27ff,
    0x02ea, 0x0b87, 0x022c, 0x089e, 0x0ec2, 0x01e1, 0x05f2, 0x0d94,
    0x01e1, 0x09b7, 0x0cc2, 0x1601, 0x01e8, 0x0d2d, 0x1929, 0x0d10,
    0x0011, 0x3b01, 0x05d2, 0x103a, 0x07f4, 0x075a, 0x0715, 0x01d3,
    0x0ceb, 0x36da, 0x18e3, 0x0292, 0x03ed, 0x0387, 0x02e1, 0x075f,
    0x1d17, 0x0760, 0x0b20, 0x06f8, 0x1d87, 0x0d48, 0x03b7, 0x3691,
    0x10d0, 0x00b1, 0x0029, 0x4da3, 0x0c26, 0x33a5, 0x2216, 0x023b,
    0x1b83, 0x1b1f, 0x04af, 0x0160, 0x1923, 0x00a5, 0x0491, 0x0cf3,
    0x03d2, 0x00e9, 0x0bbb, 0x0a02, 0x0bb2, 0x295b, 0x272e, 0x0949,
    0x076e, 0x14ea, 0x115f, 0x0613, 0x0107, 0x6993, 0x08eb, 0x0131,
    0x029d, 0x0778, 0x0259, 0x182a, 0x01ad, 0x078a, 0x3a19, 0x06f8,
    0x067d, 0x020c, 0x0df9, 0x00ec, 0x0938, 0x1802, 0x0b22, 0xd955,
    0x06d9, 0x1052, 0x2112, 0x00de, 0x0a13, 0x0ab7, 0x07ef, 0x08b2,
    0x08e4, 0x0176, 0x0854, 0x032d, 0x5cec, 0x064a, 0x1146, 0x1427,
    0x06bd, 0x0e0d, 0x0d26, 0x3800, 0x0243, 0x00a5, 0x055f, 0x2722,
    0x3148, 0x2658, 0x055b, 0x0218, 0x074b, 0x2a70, 0x0359, 0x089e,
    0x169c, 0x01b2, 0x1f95, 0x44d2, 0x02d7, 0x0e37, 0x063b, 0x1350,
    0x0851, 0x07ed, 0x2003, 0x2098, 0x1858, 0x23df, 0x1fbe, 0x074e,
    0x0ce0, 0x1d1f, 0x22f3, 0x61b9, 0x021d, 0x4aab, 0x0170, 0x0236,
    0x162a, 0x019b, 0x020a, 0x0403, 0x2017, 0x0802, 0x1990, 0x2741,
    0x0266, 0x0306, 0x091d, 0x0bbf, 0x8981, 0x1262, 0x0480, 0x06f9,
    0x0404, 0x0604, 0x0e9f, 0x01ed, 0x117a, 0x09d9, 0x68dd, 0x20a2,
    0x0360, 0x49e3, 0x1559, 0x098f, 0x002a, 0x119f, 0x067c, 0x00a6,
    0x04e1, 0x1873, 0x09f9, 0x0130, 0x0110, 0x1c76, 0x0049, 0x199a,
    0x0383, 0x0b00, 0x144d, 0x3412, 0x1b8e, 0x0b02, 0x0c7f, 0x032b,
    0x039a, 0x015e, 0x1d5a, 0x1164, 0x0d79, 0x0a67, 0x1264, 0x01a2,
    0x0655, 0x0493, 0x0d8f, 0x0058, 0x2c51, 0x019c, 0x0617, 0x00c2,
];

// http://miller-rabin.appspot.com/
// http://web.archive.org/web/20220921163920/http://www.janfeitsma.nl/math/psp2/index
fn is_sprp_64(n: u64, a: u64) -> bool {
    let s = (n - 1).trailing_zeros();
    let d = n >> s;
    let mut cur = {
        let mut cur = 1;
        let mut pow = d;
        let mut a = a;
        while pow > 0 {
            if pow & 1 != 0 {
                cur = (cur as u128 * a as u128 % n as u128) as u64;
            }
            a = ((a as u128).pow(2) % n as u128) as u64;
            pow >>= 1;
        }
        cur
    };
    if cur == 1 {
        return true;
    }
    for _ in 0..s {
        if cur == n - 1 {
            return true;
        }
        cur = ((cur as u128).pow(2) % n as u128) as u64;
    }
    false
}

#[test]
fn exhaustive_u8() {
    let is_prime_naive = |x: u8| x > 1 && (2..x).all(|y| x % y > 0);

    for i in 0..=255 {
        assert_eq!(i.is_prime(), is_prime_naive(i), "{i}");
    }
}

#[test]
fn exhaustive_u16() {
    let w = 64;
    let n = w << 10;
    let is_prime = {
        let mut dp = vec![!0_u64; n / w + 1];
        dp[0] &= !0 << 2;
        for i in (2..=n).take_while(|&i| i <= n / i) {
            let (qi, ri) = (i / w, i % w);
            if dp[qi] >> ri & 1 == 0 {
                continue;
            }
            for j in i..=n / i {
                let (qj, rj) = (i * j / w, i * j % w);
                dp[qj] &= !(1 << rj);
            }
        }
        dp
    };

    for i in 2..n {
        let actual = (i as u16).is_prime();
        let expected = is_prime[i / w] >> (i % w) & 1 != 0;
        assert_eq!(actual, expected, "{i}");
    }
}

#[test]
fn exhaustive_u32() {
    let w = 64;
    let n = w << 26; // takes ~30s
    let is_prime = {
        let mut dp = vec![!0_u64; n / w + 1];
        dp[0] &= !0 << 2;
        for i in (2..=n).take_while(|&i| i <= n / i) {
            let (qi, ri) = (i / w, i % w);
            if dp[qi] >> ri & 1 == 0 {
                continue;
            }
            for j in i..=n / i {
                let (qj, rj) = (i * j / w, i * j % w);
                dp[qj] &= !(1 << rj);
            }
        }
        dp
    };

    for i in 2..n {
        let actual = (i as u32).is_prime();
        let expected = is_prime[i / w] >> (i % w) & 1 != 0;
        assert_eq!(actual, expected, "{i}");
    }
}

#[test]
fn small_u64() {
    let w = 64;
    let n = w << 18;
    let is_prime = {
        let mut dp = vec![!0_u64; n / w + 1];
        dp[0] &= !0 << 2;
        for i in (2..=n).take_while(|&i| i <= n / i) {
            let (qi, ri) = (i / w, i % w);
            if dp[qi] >> ri & 1 == 0 {
                continue;
            }
            for j in i..=n / i {
                let (qj, rj) = (i * j / w, i * j % w);
                dp[qj] &= !(1 << rj);
            }
        }
        dp
    };

    for i in 2..n {
        let actual = (i as u64).is_prime();
        let expected = is_prime[i / w] >> (i % w) & 1 != 0;
        assert_eq!(actual, expected, "{i}");
    }
}

#[test]
fn mul_u64() {
    let primes = [2, 3, 5, 13, 19, 73, 193, 407521, 299210837];
    let max = u64::MAX;
    let mult = {
        let n = primes.len();
        let mut mult = vec![];
        let mut q: Vec<_> = (0..n).map(|i| (primes[i], i)).collect();
        while let Some((x, i)) = q.pop() {
            for j in i..n {
                let p = primes[j];
                if x > max / p {
                    continue;
                }
                let y = x * p;
                q.push((y, j));
                mult.push(y);
            }
        }
        mult
    };

    assert!(mult.iter().all(|&x| !x.is_prime()));
}
