#[cfg(test)]
mod tests_judge {
    use verifiers::solver::*;
    use verifiers::test_set::verify;

    #[test]
    fn test_ac() {
        verify::<Aoj0000>();
    }

    #[test]
    #[should_panic(expected = "WA")]
    fn test_wa() {
        verify::<Aoj0000Wa>();
    }

    #[test]
    #[should_panic(expected = "RE")]
    fn test_re() {
        verify::<Aoj0000Re>();
    }

    #[test]
    #[should_panic(expected = "TLE")]
    fn test_tle() {
        verify::<Aoj0000Tle>();
    }

    #[test]
    #[should_panic(expected = "no cases")]
    fn test_no() {
        verify::<Aoj0002>();
    }
}
