use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [i64; n],
        xy: [(i64, i64); q],
    }

    aa.sort_unstable();

    let solve = |x: i64, y: i64| {
        let start = aa.lower_bound(&x);

        let mut ok = 3 * 10_i64.pow(10);
        let mut ng = 0_i64;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;
            if mid - x + 1 - aa[start..].upper_bound(&mid) as i64 >= y {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    };

    let output = xy.iter().map(|&(x, y)| solve(x, y)).join("\n");
    println!("{output}");
}
