use bisect::bisect;

use crate::jury;
use crate::test_set::*;

pub struct Aoj0270 {}

impl Solver for Aoj0270 {
    type Jury = jury::Aoj0270;
    fn solve((mut c, qs): (Vec<u32>, Vec<u32>)) -> Vec<u32> {
        c.push(0);
        c.sort_unstable();
        let c = c;
        let n = c.len();

        qs.into_iter()
            .map(|q| {
                let mut res = c[n - 1] % q;
                for i in 1.. {
                    let y = i * q;
                    let pred = |&x: &u32| x < y;
                    let b = bisect(&c, pred);
                    if b == n {
                        break;
                    }
                    res = res.max(c[b - 1] % q);
                }
                res
            })
            .collect()
    }
}
