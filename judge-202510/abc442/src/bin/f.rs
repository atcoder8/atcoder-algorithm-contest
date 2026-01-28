use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n],
    }

    let mut dp = vec![0_usize; n + 1];
    let mut next_dp = vec![0_usize; n + 1];
    for s in &ss {
        let mut acc_min_cost = dp[n];
        let mut add_cost = s.iter().filter(|&&ch| ch == '#').count();
        next_dp[n] = acc_min_cost + add_cost;
        for (i, &ch) in enumerate(s).rev() {
            acc_min_cost = acc_min_cost.min(dp[i]);
            if ch == '#' {
                add_cost -= 1;
            } else {
                add_cost += 1;
            }

            next_dp[i] = acc_min_cost + add_cost;
        }

        std::mem::swap(&mut dp, &mut next_dp);
    }

    let min_cost = *dp.iter().min().unwrap();
    println!("{min_cost}");
}
