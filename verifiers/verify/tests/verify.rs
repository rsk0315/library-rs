#[cfg(test)]
mod verify {
    use verify::solver::*;
    use verify::test_set::verify;

    use op_add::OpAdd;
    use potentialized_union_find::PotentializedUnionFind;
    use union_find::UnionFind;

    #[test]
    fn verify_bisect() {
        verify::<Aoj0270>();
    }

    #[test]
    fn verify_tortoise_hare() {
        verify::<Aoj1180>();
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

    #[test]
    fn verify_scc() {
        verify::<AojGrl3C>();
    }

    #[test]
    fn verify_kmp() {
        verify::<AojAldsOne14BKmp>();
    }

    #[test]
    fn verify_z() {
        verify::<AojAldsOne14BZ>();
    }
    #[test]
    fn verify_potentialized_uf() {
        verify::<AojDsl1B<PotentializedUnionFind<OpAdd<i32>>>>();
    }
}
