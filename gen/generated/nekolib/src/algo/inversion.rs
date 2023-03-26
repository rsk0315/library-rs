pub trait Inversion {
    fn inversion(&self) -> u64;
}

impl<T: Ord> Inversion for [T] {
    fn inversion(&self) -> u64 {
        let n = self.len();
        let ord = {
            let mut ord: Vec<_> = (0..n).collect();
            ord.sort_unstable_by(|&il, &ir| {
                self[il].cmp(&self[ir]).then_with(|| il.cmp(&ir))
            });
            ord
        };

        let mut res = 0;
        let mut sum = vec![0; n + 1];
        for i in ord.iter().map(|&i| i + 1) {
            {
                let mut i = i;
                while i <= n {
                    res += sum[i];
                    i += i & i.wrapping_neg();
                }
            }
            {
                let mut i = i;
                while i > 0 {
                    sum[i] += 1;
                    i -= i & i.wrapping_neg();
                }
            }
        }

        res
    }
}

#[test]
fn sanity_check() {
    assert_eq!([1, 5, 4, 2, 3].inversion(), 5);
    assert_eq!([1, 2, 3, 4, 5].inversion(), 0);
    assert_eq!([5, 4, 3, 2, 1].inversion(), 10);
    assert_eq!([1, 1, 1, 1, 1].inversion(), 0);

    let empty: [(); 0] = [];
    assert_eq!(empty.inversion(), 0);
}
