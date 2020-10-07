use tortoise_hare::tortoise_hare;

fn parse_aoj_1180(input: String) -> Vec<(u32, usize)> {
    input
        .lines()
        .filter_map(|s| {
            let mut ss = s.split(" ");
            let a = ss.next().unwrap().parse().unwrap();
            let l = ss.next().unwrap().parse().unwrap();
            match (a, l) {
                (0, 0) => None,
                (a, l) => Some((a, l)),
            }
        })
        .collect()
}

pub fn aoj_1180(input: String) -> String {
    let al = parse_aoj_1180(input);
    let mut output = "".to_string();
    for (a, l) in al {
        let f = |a| {
            let s = format!("{0:01$}", a, l);
            let mut s: Vec<_> = s.as_str().chars().collect();
            s.sort();
            let s0: u32 = s.iter().collect::<String>().parse().unwrap();
            let s1: u32 = s.iter().rev().collect::<String>().parse().unwrap();
            s1 - s0
        };
        let (mu, lambda) = tortoise_hare(a, f);
        let a = std::iter::successors(Some(a), |&x| Some(f(x)))
            .skip(mu)
            .next()
            .unwrap();
        output.push_str(&format!("{} {} {}\n", mu, a, lambda));
    }
    output
}
