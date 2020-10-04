use std::ops::{Index, Range};

use fold::Fold;
use op_add::OpAdd;
use set_value::SetValue;

enum Query {
    Add(usize, u64),
    GetSum(usize, usize),
}

fn parse_aoj_dsl_2_b(input: String) -> (usize, Vec<Query>) {
    let mut input = input.lines();
    let (n, q) = {
        let tmp: Vec<_> = input
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        (tmp[0], tmp[1])
    };
    let qs = (0..q)
        .map(|_| {
            let mut ss = input.next().unwrap().split(" ");
            match ss.next().unwrap() {
                "0" => {
                    let i = ss.next().unwrap().parse::<usize>().unwrap() - 1;
                    let x = ss.next().unwrap().parse().unwrap();
                    Query::Add(i, x)
                }
                "1" => {
                    let s = ss.next().unwrap().parse::<usize>().unwrap() - 1;
                    let t = ss.next().unwrap().parse().unwrap();
                    Query::GetSum(s, t)
                }
                _ => unreachable!(),
            }
        })
        .collect();
    (n, qs)
}

pub fn aoj_dsl_2_b<Q>(input: String) -> String
where
    Q: From<Vec<u64>>
        + SetValue<usize, Input = u64>
        + Index<usize, Output = u64>
        + Fold<Range<usize>, Output = OpAdd<u64>>,
{
    let (n, qs) = parse_aoj_dsl_2_b(input);
    let mut output = "".to_string();

    let mut rq: Q = vec![0u64; n].into();
    for q in qs {
        match q {
            Query::Add(i, x) => rq.set_value(i, rq[i] + x),
            Query::GetSum(s, t) => {
                output.push_str(&format!("{}\n", rq.fold(s..t)))
            }
        }
    }

    output
}
