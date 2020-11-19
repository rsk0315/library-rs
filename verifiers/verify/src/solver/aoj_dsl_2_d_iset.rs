use std::ops::Range;

use crate::jury;
use crate::test_set::Solver;

use jury::aoj_dsl_2_d::Query;

use interval_set::IntervalSet;

pub struct AojDsl2DIset {}

impl Solver for AojDsl2DIset {
    type Jury = jury::AojDsl2D;
    fn solve((n, qs): (usize, Vec<Query>)) -> Vec<u32> {
        let w = 31;
        let mut iset = vec![IntervalSet::<usize>::new(); w];

        for wi in 0..w {
            iset[wi].insert(0..n);
        }

        qs.into_iter()
            .filter_map(|q| match q {
                Query::Update(Range { start: s, end: t }, x) => {
                    // eprintln!("update {:?} to {}", s..t, x);
                    for wi in 0..w {
                        if x >> wi & 1 == 0 {
                            iset[wi].remove(s..t);
                        } else {
                            iset[wi].insert(s..t);
                        }
                    }
                    None
                }
                Query::Find(i) => {
                    // eprintln!("what value of [{}]", i);
                    let res = (0..w)
                        .filter_map(|wi| {
                            if !iset[wi].is_empty() {
                                // eprintln!("iset[{}]: {:#?}", wi, iset[wi]);
                            }
                            if iset[wi].has_range(&(i..=i)) {
                                // eprintln!("has 2 ^ {}", wi);
                                Some(1_u32 << wi)
                            } else {
                                None
                            }
                        })
                        .sum::<u32>();
                    Some(res)
                }
            })
            .collect()
    }
}
