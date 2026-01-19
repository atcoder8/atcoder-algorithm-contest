use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        mut aa: [usize; n],
        xy: [(usize, usize); q],
    }

    aa.sort_unstable();

    let solve = |x: usize, y: usize| {
        let start = aa.lower_bound(&x);

        if start == n || aa[start] - x >= y {
            return x + y - 1;
        }

        let mut ok = n;
        let mut ng = start;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;
            if aa[mid] + start >= x + y + mid {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        x + y - 1 + ok - start
    };

    let output = xy.iter().map(|&(x, y)| solve(x, y)).join("\n");
    println!("{output}");
}
