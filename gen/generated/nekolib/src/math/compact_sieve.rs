//! 篩。

/// 篩。
pub struct CompactSieve {
    n: usize,
    is_prime: Vec<u64>,
}

const WORD_SIZE: usize = 64;

impl CompactSieve {
    pub fn new(n: usize) -> Self {
        let mut is_prime = vec![0xAAAAAAAAAAAAAAAA; 1 + n / WORD_SIZE];
        is_prime[0] |= 1 << 2;
        for i in (3..=n).take_while(|&i| i * i <= n) {
            if is_prime[i / WORD_SIZE] >> (i % WORD_SIZE) & 1 == 0 {
                continue;
            }
            for j in i..=n / i {
                let ij = i * j;
                is_prime[ij / WORD_SIZE] &= !(1_u64 << (ij % WORD_SIZE));
            }
        }
        Self { n, is_prime }
    }
    pub fn is_prime(&self, i: usize) -> bool {
        self.is_prime[i / WORD_SIZE] >> (i % WORD_SIZE) & 1 == 1
    }
    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        (2..=self.n).filter(move |&i| self.is_prime(i))
    }
}
