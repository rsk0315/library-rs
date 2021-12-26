fn main() {}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use std::ops::Add;

    use act::Act;
    use action::MonoidAction;
    use additive::{AddAssoc, Zero};
    use fold::Fold;
    use min::Min;
    use op_add::OpAdd;
    use op_max::OpMax;
    use vec_act_segtree::VecActSegtree;

    // 手軽に作れるように、OpClosure と ActClosureToClosure みたいなのを
    // 用意しておく。

    #[derive(Clone, Copy, Default)]
    struct ActAddToMax<T: Ord + Eq + Min + Add + AddAssoc + Zero + Sized> {
        op_add: OpAdd<T>,
        op_max: OpMax<T>,
        _t: PhantomData<T>,
    }

    impl<T: Ord + Eq + Min + Add + AddAssoc + Zero + Sized> MonoidAction
        for ActAddToMax<T>
    {
        type Operand = OpMax<T>;
        type Operator = OpAdd<T>;
        fn operand(&self) -> &Self::Operand { &self.op_max }
        fn operator(&self) -> &Self::Operator { &self.op_add }
        fn act(&self, x: T, op: T) -> T { x + op }
    }

    #[test]
    fn test1() {
        let mut a: VecActSegtree<ActAddToMax<i32>> =
            vec![1_i32, 6, 2, 8, 6, 3].into();
        assert_eq!(8, a.fold(2..5));
        a.act(2..4, -5);
        // [1, 6, -3, 3, 6, 3]
        assert_eq!(6, a.fold(2..5));
    }

    #[test]
    fn test2() {
        let n = 10000;
        let mut st: VecActSegtree<ActAddToMax<i128>> = vec![0; n].into();
        let mut a = vec![0_i128; n];
        let mut it =
            std::iter::successors(Some(3_usize), |x| Some(3 * x % 46337));
        for x in 1..=100000 {
            let i1 = it.next().unwrap() % n;
            let i2 = it.next().unwrap() % n;
            let l = i1.min(i2);
            let r = i1.max(i2) + 1;
            st.act(l..r, x);
            for i in l..r {
                a[i] += x;
            }

            let i1 = it.next().unwrap() % n;
            let i2 = it.next().unwrap() % n;
            let l = i1.min(i2);
            let r = i1.max(i2) + 1;
            let actual = st.fold(l..r);
            let expected = a[l..r].iter().copied().max().unwrap();
            assert_eq!(actual, expected);
        }
    }
}
