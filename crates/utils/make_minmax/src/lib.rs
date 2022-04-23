pub trait MakeMin: PartialOrd + Sized {
    fn make_min(&mut self, other: Self) -> bool {
        let tmp = *self > other;
        if tmp {
            *self = other;
        }
        tmp
    }
}

pub trait MakeMax: PartialOrd + Sized {
    fn make_max(&mut self, other: Self) -> bool {
        let tmp = *self < other;
        if tmp {
            *self = other;
        }
        tmp
    }
}

impl<T: PartialOrd + Sized> MakeMin for T {}
impl<T: PartialOrd + Sized> MakeMax for T {}

#[test]
fn test() {
    let mut a = 10_i32;
    a.make_max(20);
    assert_eq!(a, 20);
    a.make_max(15);
    assert_eq!(a, 20);
    a.make_min(10);
    assert_eq!(a, 10);
    a.make_min(15);
    assert_eq!(a, 10);
}
