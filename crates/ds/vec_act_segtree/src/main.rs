fn main() {}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use std::ops::AddAssign;

    use act::Act;
    use action::MonoidAction;
    use additive::{AddAssoc, Zero};
    use fold::Fold;
    use min::Min;
    use op_add::OpAdd;
    use op_max::OpMax;
    use vec_act_segtree::VecActSegtree;

    struct ActAddToMax<T: Ord + Eq + Min + AddAssign + AddAssoc + Zero + Sized> {
        op_add: OpAdd<T>,
        op_max: OpMax<T>,
        _t: PhantomData<T>,
    }

    impl<T: Ord + Eq + Min + AddAssign + AddAssoc + Zero + Sized> MonoidAction
        for ActAddToMax<T>
    {
        type Operand = OpMax<T>;
        type Operator = OpAdd<T>;
        fn operand(&self) -> Operand { self.op_max }
        fn operator(&self) -> Operator { self.op_add }
        fn act(x: &mut T, op: T) { *x += op; }
    }

    #[test]
    fn test() {
        let mut a: VecActSegtree<ActAddToMax<i32>> =
            vec![1_i32, 6, 2, 8, 6, 3].into();
        assert_eq!(8, a.fold(2..5));
        a.act(2..4, -5);
        // [1, 6, -3, 3, 6, 3]
        assert_eq!(6, a.fold(2..5));
    }
}
