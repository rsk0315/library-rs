macro_rules! simple_verify {
    (($c:expr, $s:expr)) => {
        Verifier::new()
            .testcases(Path::new($c).to_path_buf())
            .solver($s)
            .run();
    };
    ( $( ($c:expr, $s:expr), )* ) => { $( simple_verify!(($c, $s)) )* };
    ( $( ($c:expr, $s:expr) ),* ) => { $( simple_verify!(($c, $s)) )* };
}

macro_rules! simple_test {
    (
        $( $( #[$m:meta] )* ($t:ident, $c:expr, $s:expr), )*
    ) => {
        $(
            #[test]
            $( #[$m] )*
            fn $t() { simple_verify!(($c, $s)); }
        )*
    }
}

#[cfg(test)]
mod tests_judge {
    use std::path::Path;

    use verifiers::judge::Verifier;
    use verifiers::solver::aoj::*;

    simple_test! {
        (test_aoj_0000_ac, "testcases/aoj/0000", &aoj_0000),

        #[should_panic(expected = "RE")]
        (test_aoj_0000_re, "testcases/aoj/0000", &aoj_0000_re),

        #[should_panic(expected = "TLE")]
        (test_aoj_0000_tle, "testcases/aoj/0000", &aoj_0000_tle),

        #[should_panic(expected = "WA")]
        (test_aoj_0000_wa, "testcases/aoj/0000", &aoj_0000_wa),

        #[should_panic(expected = "no cases")]
        (test_aoj_0002_no, "testcases/aoj/0002", &aoj_0002),
    }
}

#[cfg(test)]
mod tests_verify {
    use std::path::Path;
    use std::time::Duration;

    use verifiers::judge::Verifier;
    use verifiers::solver::aoj::*;

    use vec_segtree::*;

    #[test]
    fn verify_vec_segtree() {
        simple_verify! {
            ("testcases/aoj/DSL_2_B", &aoj_dsl_2_b::<VecSegtree<_>>),
        }
    }

    #[test]
    fn verify_partition_point() {
        Verifier::new()
            .testcases(Path::new("testcases/aoj/0270").to_path_buf())
            .solver(&aoj_0270)
            .tl(Duration::from_millis(3000))
            .run();
    }
}
