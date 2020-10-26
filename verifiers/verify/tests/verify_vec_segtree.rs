#[cfg(test)]
mod verify_vec_segtree {
    use verify::solver::*;
    use verify::test_set::verify;

    use vec_segtree::*;

    #[test]
    fn verify_with_aoj_0564() {
        verify::<Aoj0564<VecSegtree<_>>>();
    }

    #[test]
    fn verify_with_aoj_dsl_1_2_b() {
        verify::<AojDsl2B<VecSegtree<_>>>();
    }

    #[test]
    fn verify_yuki_3287() {
        verify::<Yuki3287<VecSegtree<_>, VecSegtree<_>>>();
    }
}
