use super::super::ds::vec_segtree;
use super::super::traits::fold;
use super::super::traits::get_mut;
use super::super::utils::op_add;
use fold::Fold;
use get_mut::GetMut;
use op_add::OpAdd;
use vec_segtree::VecSegtree;

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

        let mut st: VecSegtree<OpAdd<_>> = vec![0; n].into();
        let mut res = 0;
        for &i in &ord {
            res += st.fold(i..);
            *st.get_mut(i).unwrap() += 1;
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
