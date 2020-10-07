macro_rules! default_verify {
    (($c:expr, $s:expr, $t:expr)) => {
        Verifier::new()
            .testcases(Path::new($c).to_path_buf())
            .solver($s)
            .run();
    };
    (($c:expr, $s:expr)) => { default_verify!(($c, $s, 2000)) };
    ( $( ( $( $c:expr ),* ), )* ) => { $( default_verify!(( $( $c ),* )) );* };
    ( $( ( $( $c:expr ),* ) ),* ) => { $( default_verify!(( $( $c ),* )) );* };
}

macro_rules! default_single_tests {
    (
        $( $( #[$m:meta] )* ($t:ident, $( $c:expr ),*), )*
    ) => {
        $(
            #[test]
            $( #[$m] )*
            fn $t() { default_verify!(($( $c ),*)); }
        )*
    }
}

#[cfg(test)]
mod tests_judge {
    use std::path::Path;

    use verifiers::judge::Verifier;
    use verifiers::solver::aoj::*;

    default_single_tests! {
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

    use verifiers::judge::Verifier;
    use verifiers::solver::aoj::*;
    use verifiers::solver::yukicoder::*;

    use vec_segtree::*;

    #[test]
    fn verify_vec_segtree() {
        default_verify! {
            ("testcases/aoj/DSL_2_B", &aoj_dsl_2_b::<VecSegtree<_>>),
            ("testcases/yukicoder/3287", &yuki_3287::<VecSegtree<_>, VecSegtree<_>>),
            ("testcases/aoj/0564", &aoj_0564::<VecSegtree<_>>, 8000),
        }
    }

    default_single_tests! {
        (verify_bisect, "testcases/aoj/0270", &aoj_0270, 3000),
        (test_tortoise_hair, "testcases/aoj/1180", &aoj_1180, 8000),
    }
}
