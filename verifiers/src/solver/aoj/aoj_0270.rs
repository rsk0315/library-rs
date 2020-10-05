use partition_point::partition_point;

fn parse_aoj_0270(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut input = input.lines();
    let (_n, q) = {
        let v: Vec<_> = input
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        (v[0], v[1])
    };

    let c = input
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();
    let qs = input.take(q).map(|x| x.parse().unwrap()).collect();
    (c, qs)
}

pub fn aoj_0270(input: String) -> String {
    let (mut c, qs) = parse_aoj_0270(input);
    c.push(0);
    c.sort_unstable();
    let c = c;
    let n = c.len();

    let mut output = "".to_string();
    for q in qs {
        let mut res = c[n - 1] % q;
        for i in 1.. {
            let y = i * q;
            let pred = |&x: &i32| x < y;
            let pp = partition_point(&c, pred);
            if pp == n {
                break;
            }
            res = res.max(c[pp - 1] % q);
        }
        output.push_str(&format!("{}\n", res));
    }
    output
}
