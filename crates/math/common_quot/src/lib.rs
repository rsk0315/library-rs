//! 商が共通の区間の列挙。

/// 商が共通の区間の列挙。
///
/// $\gdef\floor#1{\\lfloor#1\\rfloor}$
/// 以下の条件を満たす $(i\_l, i\_r)$ を $i\_l$ の昇順に列挙する。
/// - $1\\le i\_l \\wedge i\_r \\le n$,
/// - $j\\in [i\_l, i\_r] \\implies \\floor{n/j} = \\floor{n/i\_l}$, and
/// - $j\\notin [i\_l, i\_r] \\implies j=0 \\vee \\floor{n/j} \\ne \\floor{n/i\_l}$.
///
/// # Complexity
/// $O(\\sqrt{n})$ time, $O(1)$ space.
///
/// # Examples
/// ```
/// use nekolib::math::CommonQuot;
///
/// assert_eq!(
///     60_u64.common_quot().collect::<Vec<_>>(),
///     [
///         (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8),
///         (9, 10), (11, 12), (13, 15), (16, 20), (21, 30), (31, 60)
///     ]
/// );
/// ```
///
/// ## See also
/// - [ABC 044 D](https://atcoder.jp/contests/abc044/tasks/arc060_b)
pub trait CommonQuot {
    type Output;
    fn common_quot(self) -> Self::Output;
}

pub struct CommonQuotStruct<I> {
    i: I,
    n: I,
}

macro_rules! impl_common_quot_unit {
    ( $($ty:ty)* ) => { $(
        impl CommonQuot for $ty {
            type Output = CommonQuotStruct<$ty>;
            fn common_quot(self) -> Self::Output {
                Self::Output { i: 1, n: self }
            }
        }
        impl Iterator for CommonQuotStruct<$ty> {
            type Item = ($ty, $ty);
            fn next(&mut self) -> Option<($ty, $ty)> {
                if self.i <= self.n {
                    let l = self.i;
                    let q = self.n / l;
                    let r = self.n / q;
                    self.i = r + 1;
                    return Some((l, r));
                }
                None
            }
        }
    )* };
}

impl_common_quot_unit! { u8 u16 u32 u64 u128 usize }

#[test]
fn test_small() {
    let suite: &[(u64, &[(u64, u64)])] = &[
        (0, &[]),
        (1, &[(1, 1)]),
        (2, &[(1, 1), (2, 2)]),
        (3, &[(1, 1), (2, 3)]),
        (4, &[(1, 1), (2, 2), (3, 4)]),
        (5, &[(1, 1), (2, 2), (3, 5)]),
        (10, &[(1, 1), (2, 2), (3, 3), (4, 5), (6, 10)]),
        (100, &[
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 10),
            (11, 11),
            (12, 12),
            (13, 14),
            (15, 16),
            (17, 20),
            (21, 25),
            (26, 33),
            (34, 50),
            (51, 100),
        ]),
        (200, &[
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (9, 9),
            (10, 10),
            (11, 11),
            (12, 12),
            (13, 13),
            (14, 14),
            (15, 15),
            (16, 16),
            (17, 18),
            (19, 20),
            (21, 22),
            (23, 25),
            (26, 28),
            (29, 33),
            (34, 40),
            (41, 50),
            (51, 66),
            (67, 100),
            (101, 200),
        ]),
    ];
    for (n, expected) in suite {
        let actual: Vec<_> = n.common_quot().collect();
        assert_eq!(&actual, expected);
    }
}

#[test]
fn test() {
    let n_max = 10000_u64;
    for n in 1..=n_max {
        let mut l = 1;
        let mut expected = vec![];
        while l <= n {
            let r = (l..).take_while(|&r| n / r == n / l).last().unwrap();
            expected.push((l, r));
            l = r + 1;
        }
        let actual: Vec<_> = n.common_quot().collect();
        if n == 60 {
            eprintln!("{:?}", actual);
        }
        assert_eq!(actual, expected);
    }
}
