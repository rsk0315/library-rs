#[cfg(test)]
mod verify {
    use verify::solver::*;
    use verify::test_set::verify;

    use union_find::*;
    use vec_segtree::*;

    #[test]
    fn verify_bisect() {
        verify::<Aoj0270>();
    }

    #[test]
    fn verify_tortoise_hare() {
        verify::<Aoj1180>();
    }

    #[test]
    fn verify_vec_segtree() {
        verify::<Aoj0564<VecSegtree<_>>>();
        verify::<AojDsl2B<VecSegtree<_>>>();
        verify::<Yuki3287<VecSegtree<_>, VecSegtree<_>>>();
    }

    #[test]
    fn verify_union_find() {
        verify::<AojDsl1A<UnionFind>>();
    }

    #[test]
    fn verify_mo() {
        verify::<Aoj0425<aoj_0425::Neko>>();
    }

    #[test]
    fn verify_dijkstra() {
        verify::<AojGrl1A>();
    }

    #[test]
    fn verify_parallel_bisect() {
        verify::<Aoj0575<UnionFind>>();
    }
}
