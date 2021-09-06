//! 組合せのビット表現。

/// 組合せのビット表現。
///
/// $n$ bit で表せる整数のうち、$k$ 個の bit が `1` であるものを昇順に生成する。
///
/// # Idea
///
/// $k$ 個の bit が `1` である整数 `i` が与えられたとき、$k$ 個の bit が `1`
/// である整数のうち `i` より大きく最小の整数 `j` を考える。
///
/// 例として `i = 011001110` とする。
/// ```text
/// 011001110  // i
///     ^~~~
/// ```
/// `i` のうちで `1` が連続して現れる部分のうち、最も右のものを考える。
/// これの左にある `0` の位は、`j` においては `1` である必要がある。
/// ```text
/// 011001110  // i
/// 01101....  // j (upper)
/// ```
/// また、残った下位の領域においては、`i` における `1` の連続個数よりひとつ少ない
/// `1` を右詰めで入れればよい。
/// ```text
/// 011001110  // i
/// .....0011  // j (lower)
/// ```
///
/// あとは、これらを計算する方法について述べる。
/// ```text
/// 011001110  // i
/// 000000010  // x = i & i.wrapping_neg()
/// 011010000  // y = i + x
/// 000001110  // i & !y
/// 000000011  // z = (i & !y) >> (x.trailing_zeros() + 1)
/// 011010011  // j = y | z
/// ```
/// 上記では `_ >> (x.trailing_zeros() + 1)` としているが、`x` が 2
/// べきなので `(_ / x) >> 1` と等しい。
///
/// # Examples
/// ```
/// use nekolib::math::bit_binom;
///
/// let mut it = bit_binom(4, 2);
/// assert_eq!(it.next(), Some(0b_0011));
/// assert_eq!(it.next(), Some(0b_0101));
/// assert_eq!(it.next(), Some(0b_0110));
/// assert_eq!(it.next(), Some(0b_1001));
/// assert_eq!(it.next(), Some(0b_1010));
/// assert_eq!(it.next(), Some(0b_1100));
/// assert_eq!(it.next(), None);
/// ```
///
/// # References
/// - 蟻本 p.144
pub fn bit_binom(n: usize, k: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(!(!0_usize << k)), move |&i| {
        if k == 0 {
            return None;
        }
        let x = i & i.wrapping_neg();
        let y = i + x;
        let z = (i & !y) >> (x.trailing_zeros() + 1);
        Some(y | z)
    })
    .take_while(move |&i| i < (1_usize << n))
}
