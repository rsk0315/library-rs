use std::collections::BTreeMap;

pub struct RangeTree<T: Clone + Ord> {
    buf: Vec<T>,
    occ: BTreeMap<T, Vec<usize>>,
    ind: RangeTreeUsize,
    val: RangeTreeUsize,
}

impl<T: Clone + Ord> From<Vec<T>> for RangeTree<T> {
    fn from(buf: Vec<T>) -> Self {
        let occ = {
            let mut occ = BTreeMap::new();
            for (i, x) in buf.iter().cloned().enumerate() {
                occ.entry(x).or_insert(vec![]).push(i);
            }
            occ
        };
        let n = buf.len();
        let ind = {
            let mut ind = vec![0; n];
            let mut enc_x = 0;
            for v in occ.values() {
                for &i in v {
                    ind[i] = enc_x;
                    enc_x += 1;
                }
            }
            ind
        };
        let val = {
            let mut val = vec![0; n];
            for i in 0..n {
                val[ind[i]] = i;
            }
            val
        };
        let ind: RangeTreeUsize = ind.into();
        let val: RangeTreeUsize = val.into();
        Self { buf, occ, ind, val }
    }
}

// Count
// Count3way
// Quantile
// FindNth

struct RangeTreeUsize {
    casc: Vec<Vec<usize>>,
    down: Vec<Vec<(usize, usize)>>,
}

impl From<Vec<usize>> for RangeTreeUsize {
    fn from(buf: Vec<usize>) -> Self {
        let casc = vec![];
        let down = vec![];
        Self { casc, down }
    }
}
