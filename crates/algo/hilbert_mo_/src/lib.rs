//! Hilbert curve に基づく Mo's algorithm。

use std::ops::Range;

use elastic_slice::{
    ElasticSlice, ExpandBack, ExpandFront, ShrinkBack, ShrinkFront, SliceHash,
};

/// Hilbert curve に基づく Mo's algorithm。
///
/// <style>
/// .label {
///     cursor: default;
///     user-select: none !important;
///     display: inline;
///     padding: 0.2em 0.6em 0.3em;
///     font-size: 60%;
///     line-height: 1;
///     color: #fff;
///     text-align: center;
///     white-space: nowrap;
///     vertical-align: baseline;
///     border-radius: 0.25em;
///     font-family: Lato;
/// }
/// .label-warning {
///     background-color: #f0ad4e;
/// }
/// </style>
///
/// See <https://codeforces.com/blog/entry/61203>。
/// [Range Set Query](https://atcoder.jp/contests/abc174/tasks/abc174_f)
/// に投げたら <span class="label label-warning">TLE</span> したのでつらい。
pub fn hilbert_mo<S>(
    mut slice: S,
    q: Vec<(Range<usize>, S::Salt)>,
) -> impl Iterator<Item = S::Hashed>
where
    S: ElasticSlice
        + ExpandFront
        + ExpandBack
        + ShrinkFront
        + ShrinkBack
        + SliceHash,
    S::Hashed: Clone,
{
    let qn = q.len();
    let n = slice.len();
    let k = n.next_power_of_two().trailing_zeros();
    let mut q: Vec<_> = q
        .into_iter()
        .enumerate()
        .map(|(i, (Range { start, end }, x))| {
            (i, (start..end, x), ord(start, end, k))
        })
        .collect();
    q.sort_unstable_by_key(|&(.., o)| o);

    let mut res = vec![None; qn];
    slice.reset();
    for (i, (Range { start: ql, end: qr }, x), _) in q {
        while slice.end() < qr {
            slice.expand_back();
        }
        while slice.start() > ql {
            slice.expand_front();
        }
        while slice.start() < ql {
            slice.shrink_front();
        }
        while slice.end() > qr {
            slice.shrink_back();
        }
        res[i] = Some(slice.hash(x));
    }
    res.into_iter().map(std::option::Option::unwrap)
}

fn ord(i: usize, j: usize, k: u32) -> usize { ord_internal(i, j, k, 0) }

fn ord_internal(i: usize, j: usize, pow: u32, rot: usize) -> usize {
    if pow == 0 {
        return 0;
    }
    let hpow = 1 << (pow - 1);
    let seg = if i < hpow {
        if j < hpow { 0 } else { 3 }
    } else {
        if j < hpow { 1 } else { 2 }
    };
    let seg = (seg + rot) & 3;
    let drot = [3, 0, 0, 1];
    let nx = i & (i ^ hpow);
    let ny = j & (j ^ hpow);
    let nrot = (rot + drot[seg]) & 3;
    let sub = 1 << (2 * pow - 2);
    let res = seg * sub as usize;
    let add = ord_internal(nx, ny, pow - 1, nrot);
    res + if seg == 1 || seg == 2 { add } else { sub - add - 1 }
}
