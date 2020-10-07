pub fn aoj_0000(_: String) -> String {
    let mut res = "".to_string();
    for i in 1..=9 {
        for j in 1..=9 {
            res.push_str(&format!("{}x{}={}\n", i, j, i * j));
        }
    }
    res
}

pub fn aoj_0000_wa(_: String) -> String {
    "hello world".to_string()
}

pub fn aoj_0000_tle(_: String) -> String {
    let n = 10000000;
    (1..=n)
        .map(|i| (1..=i).step_by(2).sum::<u128>())
        .sum::<u128>()
        .to_string()
}

pub fn aoj_0000_re(_: String) -> String {
    panic!("nekochan");
}
