//! 区間加算 (imos 法 + on-the-fly で遅延解消)。
//!
//! 以下のような問題を考える。
//!
//! ```
//! use proconio::input;
//! # use proconio::source::auto::AutoSource;
//! # let source = AutoSource::from("4  1 3 2 5");
//!
//! input! {
//! #     from source,
//!     n: usize,
//!     a: [usize; n],
//! }
//!
//! let m = (0..n).map(|i| i + a[i]).max().unwrap();
//! let mut dp = vec![0; m + 1];
//! dp[0] = 1;
//!
//! for i in 0..n {
//!     for j in 1..=a[i] {
//!         dp[i + j] += dp[i] * j as i32;
//!     }
//! }
//!
//! // a = [1, 3, 2, 5]
//!
//! // 0: [1, 0, 0, 0, 0, 0, 0, 0, 0]
//! // 1: [1, 1, 0, 0, 0, 0, 0, 0, 0]
//! // 2: [1, 1, 1, 2, 3, 0, 0, 0, 0]
//! // 3: [1, 1, 1, 3, 5, 0, 0, 0, 0]
//! // 4: [1, 1, 1, 3, 8, 6, 9, 12, 15]
//!
//! assert_eq!(dp, [1, 1, 1, 3, 8, 6, 9, 12, 15]);
//! ```
//!
//! これを $O(n)$ 時間で行いたい。
//!
//! 1 階差分を考えると、定数の区間加算になるので、基本的な imos 法で処理できる。
//! $[i+1, i+a\_i]$ に足す値は $\\DP\[i\]$ に依るため、on-the-fly の処理が必要になる。
//!
//! $$
//! \\begin{aligned}
//! &\\qquad \\For{i \\gets (0, 1, \\dots, n-1)} \\\\
//! &\\qquad\\qquad \\For{j \\in \\{1, 2, \\dots, a\_i\\}} \\\\
//! &\\qquad\\qquad\\qquad \\DP\[i+j\] \\xgets{+} \\DP\[i\] \\cdot j \\\\
//! \\end{aligned}
//! $$
//!
//! ```
//! # use proconio::input;
//! # use proconio::source::auto::AutoSource;
//! # let source = AutoSource::from("4  1 3 2 5");
//! #
//! # input! {
//! #     from source,
//! #     n: usize,
//! #     a: [usize; n],
//! # }
//! #
//! # let m = (0..n).map(|i| i + a[i]).max().unwrap();
//! let mut dp0 = vec![0; m + 1];  // dp
//! let mut dp1 = vec![0; m + 2];  // dp'
//! let mut dp2 = vec![0; m + 3];  // dp''
//! dp2[0] = 1;
//! dp2[1] = -2;
//! dp2[2] = 1;
//!
//! for i in 0..n {
//!     dp1[i] += dp2[i];
//!     if i > 0 {
//!         dp1[i] += dp1[i - 1];
//!     }
//!     dp0[i] += dp1[i];
//!     if i > 0 {
//!         dp0[i] += dp0[i - 1];
//!     }
//!     dp2[i + a[i] + 1] -= dp0[i] * a[i] as i32;
//!     dp2[i + a[i] + 2] += dp0[i] * a[i] as i32;
//!     dp2[i + 1] += dp0[i];
//!     dp2[i + a[i] + 1] -= dp0[i];
//! }
//! for i in n..=m {
//!     dp1[i] = dp1[i - 1] + dp2[i];
//!     dp0[i] = dp0[i - 1] + dp1[i];
//! }
//!
//! assert_eq!(dp0, [1, 1, 1, 3, 8, 6, 9, 12, 15]);
//! ```
//!
//! 0 次の加算（一つの値の加算）、1 次の加算（区間への定数の加算）に関して、
//! 直接 `dp0[_]` や `dp1[_]` に足すと総和の整合性が取れなくなるので、
//! `dp2[_]` の意味で言い換えるか、別の値として持つ必要がある。
//! 前者では、無駄に足す個数が増えるので、次数が増えたときにつらそう。
//!
//! ```
//! # use proconio::input;
//! # use proconio::source::auto::AutoSource;
//! # let source = AutoSource::from("4  1 3 2 5");
//! #
//! # input! {
//! #     from source,
//! #     n: usize,
//! #     a: [usize; n],
//! # }
//! #
//! # let m = (0..n).map(|i| i + a[i]).max().unwrap();
//! #
//! let mut res = vec![0; m + 1];
//! #[allow(unused)]
//! let mut dp0 = vec![0; m + 1];
//! let mut dp1 = vec![0; m + 2];
//! let mut dp2 = vec![0; m + 2];
//! let mut acc0 = 1;
//! let mut acc1 = 0;
//!
//! dp1[1] = -acc0;
//! for i in 0..n {
//!     acc1 += dp2[i];
//!     acc0 += dp1[i] + acc1;
//!     res[i] = acc0;
//!     
//!     dp1[i + a[i] + 1] -= acc0 * a[i] as i32;
//!     dp2[i + 1] += acc0;
//!     dp2[i + a[i] + 1] -= acc0;
//! }
//! for i in n..=m {
//!     acc1 += dp2[i];
//!     acc0 += dp1[i] + acc1;
//!     res[i] = acc0;
//! }
//! assert_eq!(res, [1, 1, 1, 3, 8, 6, 9, 12, 15]);
//! ```
//!
//! ## To-do / Notes
//! - 数式からの導出をちゃんと書く
//! - $\\DP\[i+j\] \\xgets{+}\\DP\[i\]\\cdot j^2$ について書いてみる
//! - よくある累積和とは添字の解釈が異なる？ 明確にしておく
//! - クエリがオフラインで on-the-fly が不要の場合と比較してみる
//! - 遅延セグ木で区間 $O(1)$ 次加算と何かしらの fold について考える

#[test]
fn linear() {
    let a = vec![1, 3, 2, 5, 7];

    let n = a.len();
    let m = (0..n).map(|i| i + a[i]).max().unwrap();
    let expected = {
        let mut dp = vec![0; m + 1];
        dp[0] = 1;
        for i in 0..n {
            for j in 1..=a[i] {
                dp[i + j] += dp[i] * j as i32;
            }
        }
        dp
    };

    let actual = {
        let mut res = vec![0; m + 1];
        let dp0 = vec![0; m + 1];
        let mut dp1 = vec![0; m + 2];
        let mut dp2 = vec![0; m + 2];
        let mut acc0 = 1;
        let mut acc1 = 0;

        dp1[1] = -acc0;
        for i in 0..n {
            acc1 += dp2[i];
            acc0 += dp1[i] + acc1;
            res[i] = dp0[i] + acc0;

            dp1[i + a[i] + 1] -= acc0 * a[i] as i32;
            dp2[i + 1] += acc0;
            dp2[i + a[i] + 1] -= acc0;
        }
        for i in n..=m {
            acc1 += dp2[i];
            acc0 += dp1[i] + acc1;
            res[i] = dp0[i] + acc0;
        }
        res
    };

    assert_eq!(actual, expected);
}

#[test]
fn quadratic() {
    let a = vec![1, 3, 2, 5, 7];
    // let a = vec![5, 0, 0, 0, 0, 0, 0, 0, 0];

    let n = a.len();
    let m = (0..n).map(|i| i + a[i]).max().unwrap();
    let expected = {
        let mut dp = vec![0; m + 1];
        dp[0] = 1;
        for i in 0..n {
            for j in 1..=a[i] {
                dp[i + j] += dp[i] * j.pow(2) as i32;
            }
        }
        dp
    };

    // index:  0   1   2   3   4   5   6   7   8
    // a:      1   0   0   0   0   0   0   0   0
    // add:    0   1   4   9  16  25   0   0   0
    // add':   0   1   3   5   7   9 -25   0   0
    // add'':  0   1   2   2   2   2 -16  25   0
    // add''': 0   1   1   0   0   0 -18  43 -25

    let actual = {
        let mut res = vec![0; m + 1];
        let dp0 = vec![0; m + 1];
        let mut dp1 = vec![0; m + 2];
        let mut dp2 = vec![0; m + 2];
        let mut dp3 = vec![0; m + 2];
        let mut acc0 = 1;
        let mut acc1 = 0;
        let mut acc2 = 0;

        dp1[1] = -acc0;
        for i in 0..n {
            acc2 += dp3[i];
            acc1 += dp2[i] + acc2;
            acc0 += dp1[i] + acc1;
            res[i] = dp0[i] + acc0;

            // linear:
            // dp1[i + a[i] + 1] -= acc0 * a[i] as i32;
            // dp2[i + 1] += acc0;
            // dp2[i + a[i] + 1] -= acc0;

            dp2[i + a[i] + 1] -= 2 * acc0 * a[i] as i32;
            dp3[i + 1] += 2 * acc0;
            dp3[i + a[i] + 1] -= 2 * acc0;
            dp1[i + a[i] + 1] -= acc0 * a[i].pow(2) as i32;

            dp2[i + 1] += -acc0;
            dp2[i + a[i] + 1] -= -acc0;
        }
        for i in n..=m {
            acc2 += dp3[i];
            acc1 += dp2[i] + acc2;
            acc0 += dp1[i] + acc1;
            res[i] = dp0[i] + acc0;
        }
        res
    };

    assert_eq!(actual, expected);
}
