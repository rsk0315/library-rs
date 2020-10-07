#[cfg(test)]
mod verify {
    use verifiers::solver::*;
    use verifiers::test_set::verify;

    use vec_segtree::*;

    #[test]
    fn test_bisect() {
        verify::<Aoj0270>();
    }

    #[test]
    fn test_tortoise_hare() {
        verify::<Aoj1180>();
    }

    #[test]
    fn test_vec_segtree() {
        verify::<Aoj0564<VecSegtree<_>>>();
        verify::<AojDsl2B<VecSegtree<_>>>();
        verify::<Yuki3287<VecSegtree<_>, VecSegtree<_>>>();
    }
}
