#[cfg(test)]
mod verify_suffix_array {
    use verify::solver::*;
    use verify::test_set::verify;

    #[test]
    fn verify_with_aoj_alds1_14_b() {
        verify::<AojAldsOne14B>();
    }

    #[test]
    fn verify_with_aoj_alds1_14_d() {
        verify::<AojAldsOne14D>();
    }
}
