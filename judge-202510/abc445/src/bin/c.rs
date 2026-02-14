use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut dp = (0..n).collect_vec();
    for start in (0..n).rev() {
        dp[start] = dp[aa[start]];
    }

    let output = dp.iter().map(|dest| dest + 1).join(" ");
    println!("{output}");
}
