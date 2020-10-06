pub fn bisect<T, F: Fn(&T) -> bool>(buf: &[T], pred: F) -> usize {
    if buf.is_empty() || !pred(&buf[0]) {
        return 0;
    }

    let mut ok = 0;
    let mut bad = buf.len();
    while bad - ok > 1 {
        let mid = ok + (bad - ok) / 2;
        match pred(&buf[mid]) {
            true => ok = mid,
            false => bad = mid,
        }
    }
    bad
}
