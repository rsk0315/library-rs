pub fn tortoise_hare<T, F>(x: T, f: F) -> (usize, usize)
where
    T: Eq + Clone,
    F: Fn(T) -> T,
{
    let mut tor = f(x.clone());
    let mut har = f(tor.clone());

    while tor != har {
        tor = f(tor);
        har = f(f(har));
    }

    tor = x;
    let mut mu = 0;
    while tor != har {
        tor = f(tor);
        har = f(har);
        mu += 1;
    }

    let mut lambda = 1;
    har = f(tor.clone());
    while tor != har {
        har = f(har);
        lambda += 1;
    }

    (mu, lambda)
}
