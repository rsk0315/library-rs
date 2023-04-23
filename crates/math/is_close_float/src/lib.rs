pub trait IsCloseFloat {
    fn is_close(self, other: Self, rel_tol: Self, abs_tol: Self) -> bool;
}

impl IsCloseFloat for f64 {
    // See also: <https://github.com/scikit-hep/scikit-hep/blob/207cf827851d98c453c655e56bd0ee36f8f2b045/skhep/math/isclose.py#L32>
    fn is_close(self, other: f64, rel_tol: f64, abs_tol: f64) -> bool {
        assert!(
            rel_tol >= 0.0 && abs_tol >= 0.0,
            "error tolerances must be >= 0.0"
        ); // error tolerances are not NaNs.

        if self == other {
            // short-circuit; including oo == oo, -oo == -oo.
            return true;
        }
        if self.is_nan() && other.is_nan() {
            // if self and other are NaNs, judge should accept it.
            // cf. <https://atcoder.jp/contests/abc280/tasks/abc280_f>
            return true;
        }
        if self.is_nan() || other.is_nan() {
            // a number is not equal to NaN.
            return false;
        }
        let diff = (self - other).abs();
        (diff <= (rel_tol * other).abs() && diff <= (rel_tol * self).abs())
            || diff <= abs_tol
    }
}

#[test]
fn sanity_check() {
    let oo = 1.0_f64 / 0.0;
    let neg_oo = -oo;
    let nan = 0.0_f64 / 0.0;

    // numbers
    assert!(2.0_f64.is_close(3.0, 0.5, 0.0)); // |3.0 - 2.0| / 2.0
    assert!(2.0_f64.is_close(3.0, 0.0, 1.0)); // |3.0 - 2.0|
    assert!(!2.0_f64.is_close(3.0, 0.499, 0.0));
    assert!(!2.0_f64.is_close(3.0, 0.0, 0.999));

    // infinities
    assert!(oo.is_close(oo, 0.0, 0.0));
    assert!(!oo.is_close(neg_oo, 0.0, 0.0));
    assert!(neg_oo.is_close(neg_oo, 0.0, 0.0));
    assert!(oo.is_close(neg_oo, oo, oo));
    assert!(oo.is_close(2.0, oo, oo));
    assert!(!oo.is_close(2.0, 0.0, 0.0));

    // nans
    assert!(nan.is_close(nan, 0.0, 0.0));
    assert!(!nan.is_close(0.0, oo, oo));
    assert!(!nan.is_close(oo, 0.0, 0.0));
    assert!(!nan.is_close(oo, oo, oo));
}

#[test]
#[should_panic]
fn panic_nan_tol() {
    let nan = 0.0_f64 / 0.0;
    eprintln!("nan < 0.0: {:?}?", nan < 0.0);
    assert!(0.0_f64.is_close(0.0, nan, nan));
}
