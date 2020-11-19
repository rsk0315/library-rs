#[cfg(test)]
mod verify_interval_set {
    use verify::solver::*;
    use verify::test_set::verify;

    #[test]
    fn verify_with_aoj_2880() {
        verify::<Aoj2880>();
    }

    #[test]
    fn verify_with_aoj_dsl_2_d_iset() {
        verify::<AojDsl2DIset>();
    }

    #[test]
    fn verify_yuki_1601() {
        verify::<Yuki1601>();
    }
}
