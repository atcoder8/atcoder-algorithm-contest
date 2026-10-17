use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [i64; n],
        lr: [(Usize1, usize); q],
    }

    let mut ss = vec![aa[0]; n + 1];
    for i in 1..n {
        ss[i] = ss[i - 1] + aa[i];
    }
    let mut tt = vec![ss[0]; n + 1];
    for i in 1..n {
        tt[i] = tt[i - 1] + ss[i];
    }

    let calc_sum = |n: usize| {
        if n == 0 {
            return 0;
        }

        let mut ok = 1_usize;
        let mut ng = triangle_sum(n) + 1;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;
            if triangle_sum(mid) <= n {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        tt[ok - 1]
            + if triangle_sum(ok) == n {
                0
            } else {
                ss[n - triangle_sum(ok) - 1]
            }
    };

    let output = lr
        .iter()
        .map(|&(l, r)| calc_sum(r) - calc_sum(l))
        .join("\n");
    println!("{output}");
}

fn triangle_sum(n: usize) -> usize {
    n * (n + 1) / 2
}
