use std::ops::{Index, Range};

use fold::Fold;
use fold_bisect::FoldBisectRev;
use op_add::OpAdd;
use op_max::OpMax;
use set_value::SetValue;

enum Query {
    Type1(usize, usize),
}

fn parse_yuki_3287(input: String) -> (Vec<u32>, Vec<Query>) {
    let mut input = input.lines();
    let (_n, q) = {
        let tmp: Vec<_> = input
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        (tmp[0], tmp[1])
    };
    let a = input
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let qs = input
        .take(q)
        .map(|ss| {
            let mut ss = ss.split(" ");
            match ss.next().unwrap() {
                "1" => {
                    let l = ss.next().unwrap().parse::<usize>().unwrap() - 1;
                    let r = ss.next().unwrap().parse().unwrap();
                    Query::Type1(l, r)
                }
                _ => unreachable!(),
            }
        })
        .collect();
    (a, qs)
}

pub fn yuki_3287<D1, D2>(input: String) -> String
where
    D1: From<Vec<u32>>
        + Index<usize, Output = u32>
        + FoldBisectRev<Folded = OpMax<u32>>,
    D2: From<Vec<usize>>
        + SetValue<usize, Input = usize>
        + Fold<Range<usize>, Output = OpAdd<usize>>,
{
    let (a, qs) = parse_yuki_3287(input);
    let mut output = "".to_string();

    let n = a.len();
    let q = qs.len();
    let top: Vec<_> = {
        let rq: D1 = a.into();
        (0..n)
            .map(|i| match rq.fold_bisect_rev(i + 1, |x| x <= &rq[i]) {
                Some(l) => l + 1,
                None => 0,
            })
            .collect()
    };

    let js = {
        let mut tmp = vec![vec![]; n];
        for i in 0..n {
            tmp[top[i]].push(i);
        }
        tmp
    };

    let qs = {
        let mut tmp = vec![vec![]; n];
        for (iq, q) in qs.into_iter().enumerate() {
            match q {
                Query::Type1(l, r) => tmp[l].push(((l, r), iq)),
            }
        }
        tmp
    };

    let mut rq: D2 = vec![0; n].into();
    let mut res = vec![0; q];

    for i in 0..n {
        for &j in &js[i] {
            rq.set_value(j, 1);
        }
        for &((l, r), iq) in &qs[i] {
            res[iq] = rq.fold(l..r);
        }
    }

    for x in res {
        output.push_str(&format!("{}\n", x));
    }
    output
}
