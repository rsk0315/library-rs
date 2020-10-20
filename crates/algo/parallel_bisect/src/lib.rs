//! 並列二分探索。

use stateful_predicate::StatefulPred;

pub fn parallel_bisect<S: StatefulPred>(
    mut s: S,
    q: Vec<S::Input>,
) -> Vec<usize> {
    let sn = s.count();
    let qn = q.len();
    let mut ok = vec![0; qn];
    let mut bad = vec![sn + 1; qn];

    loop {
        let mut ev = vec![vec![]; sn + 1];
        let mut det = true;
        let mut it = 0..;
        for i in 0..qn {
            let j = it.next().unwrap();
            if bad[i] - ok[i] <= 1 {
                continue;
            }
            let mid = (ok[i] + bad[i]) / 2;
            ev[mid].push(j);
            det = false;
        }
        if det {
            return bad;
        }

        s.reset();
        for i in 0..=sn {
            for &j in &ev[i] {
                if s.pred(&q[j]) {
                    ok[j] = i;
                } else {
                    bad[j] = i;
                }
            }
            if i < sn {
                s.next();
            }
        }
    }
}
