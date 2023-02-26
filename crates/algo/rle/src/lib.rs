pub mod rle {
    pub struct Rle<T, I> {
        iter: I,
        last: Option<T>,
    }
    pub struct RleBy<T, I, F> {
        iter: I,
        last: Option<T>,
        compare: F,
    }
    pub struct RleByKey<T, I, K> {
        iter: I,
        last: Option<T>,
        key: K,
    }

    impl<T: PartialEq, I: Iterator<Item = T>> Rle<T, I> {
        pub fn new(iter: I) -> Self { Self { iter, last: None } }
    }
    impl<T, I: Iterator<Item = T>, F: FnMut(&T, &T) -> bool> RleBy<T, I, F> {
        pub fn new(iter: I, compare: F) -> Self {
            Self { iter, last: None, compare }
        }
    }
    impl<T, I: Iterator<Item = T>, U: PartialEq, K: FnMut(&T) -> U>
        RleByKey<T, I, K>
    {
        pub fn new(iter: I, key: K) -> Self { Self { iter, last: None, key } }
    }

    impl<T: PartialEq, I: Iterator<Item = T>> Iterator for Rle<T, I> {
        type Item = (usize, T);

        fn next(&mut self) -> Option<(usize, T)> {
            let last = self.last.take().or_else(|| self.iter.next())?;
            let mut count = 1;
            while let Some(next) = self.iter.next() {
                if last == next {
                    count += 1;
                } else {
                    self.last = Some(next);
                    return Some((count, last));
                }
            }
            Some((count, last))
        }
    }

    impl<T, I: Iterator<Item = T>, F: FnMut(&T, &T) -> bool> Iterator
        for RleBy<T, I, F>
    {
        type Item = (usize, T);

        fn next(&mut self) -> Option<(usize, T)> {
            let last = self.last.take().or_else(|| self.iter.next())?;
            let mut count = 1;
            while let Some(next) = self.iter.next() {
                if (self.compare)(&last, &next) {
                    count += 1;
                } else {
                    self.last = Some(next);
                    return Some((count, last));
                }
            }
            Some((count, last))
        }
    }

    impl<T, I: Iterator<Item = T>, U: PartialEq, K: FnMut(&T) -> U> Iterator
        for RleByKey<T, I, K>
    {
        type Item = (usize, T);

        fn next(&mut self) -> Option<(usize, T)> {
            let last = self.last.take().or_else(|| self.iter.next())?;
            let mut count = 1;
            while let Some(next) = self.iter.next() {
                if (self.key)(&last) == (self.key)(&next) {
                    count += 1;
                } else {
                    self.last = Some(next);
                    return Some((count, last));
                }
            }
            Some((count, last))
        }
    }
}

pub trait Rle<T, I> {
    fn rle(self) -> rle::Rle<T, I>;
}

pub trait RleBy<T, I> {
    fn rle_by<F: FnMut(&T, &T) -> bool>(
        self,
        compare: F,
    ) -> rle::RleBy<T, I, F>;
}

pub trait RleByKey<T, I> {
    fn rle_by_key<U: PartialEq, K: FnMut(&T) -> U>(
        self,
        key: K,
    ) -> rle::RleByKey<T, I, K>;
}

impl<T: PartialEq, I: Iterator<Item = T>> Rle<T, I> for I {
    fn rle(self) -> rle::Rle<T, I> { rle::Rle::new(self) }
}

impl<T, I: Iterator<Item = T>> RleBy<T, I> for I {
    fn rle_by<F: FnMut(&T, &T) -> bool>(
        self,
        compare: F,
    ) -> rle::RleBy<T, I, F> {
        rle::RleBy::new(self, compare)
    }
}

impl<T, I: Iterator<Item = T>> RleByKey<T, I> for I {
    fn rle_by_key<U: PartialEq, K: FnMut(&T) -> U>(
        self,
        key: K,
    ) -> rle::RleByKey<T, I, K> {
        rle::RleByKey::new(self, key)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rle, RleByKey};

    #[test]
    fn sanity_check() {
        let a = vec![1, 1, 3, 1, 2, 5, 5, 5, 5, 3];
        let rle: Vec<_> = a.into_iter().rle().collect();
        assert_eq!(rle, [(2, 1), (1, 3), (1, 1), (1, 2), (4, 5), (1, 3)]);
    }

    #[test]
    fn empty() {
        assert!(std::iter::empty::<()>().rle().next().is_none());
    }

    #[test]
    fn by_key() {
        let a = vec![1, 1, 3, 1, 2, 5, 5, 5, 5, 3];
        let rle: Vec<_> =
            a.into_iter().enumerate().rle_by_key(|&(_i, x)| x).collect();
        assert_eq!(rle, [
            (2, (0, 1)),
            (1, (2, 3)),
            (1, (3, 1)),
            (1, (4, 2)),
            (4, (5, 5)),
            (1, (9, 3)),
        ]);
    }
}
