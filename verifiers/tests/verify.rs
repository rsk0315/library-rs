#[cfg(test)]
mod tests_judge {
    use std::path::Path;

    use verifiers::judge::solve;
    use verifiers::solver::aoj::*;

    #[test]
    #[should_panic(expected = "RE")]
    fn test_aoj_0000_re() {
        solve(&Path::new("testcases/aoj/0000"), &aoj_0000_re, 2000, None);
    }

    #[test]
    #[should_panic(expected = "TLE")]
    fn test_aoj_0000_tle() {
        solve(&Path::new("testcases/aoj/0000"), &aoj_0000_tle, 2000, None);
    }

    #[test]
    #[should_panic(expected = "WA")]
    fn test_aoj_0000_wa() {
        solve(&Path::new("testcases/aoj/0000"), &aoj_0000_wa, 2000, None);
    }

    #[test]
    fn test_aoj_0000() {
        solve(&Path::new("testcases/aoj/0000"), &aoj_0000, 2000, None);
    }

    #[test]
    #[should_panic(expected = "no tests")]
    fn test_aoj_0002_no() {
        solve(&Path::new("testcases/aoj/0002"), &aoj_0002, 2000, None);
    }
}
