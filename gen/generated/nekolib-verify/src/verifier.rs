//! あーうー？

#[cfg(test)]
pub mod hello {
    use crate::solver::Aoj0000;
    use crate::test_set;

    /// Verify with [`Aoj0000`].
    ///
    /// [`Aoj0000`]: ../../solver/struct.Aoj0000.html
    #[test]
    pub fn verify_hello() {
        test_set::verify::<Aoj0000>();
    }
}
