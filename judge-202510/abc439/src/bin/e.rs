use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
    }

    let mut dp = vec![];
    for &(_, b) in ab.iter().sorted_unstable_by_key(|&&(a, b)| (a, Reverse(b))) {
        let pos = dp.partition_point(|&elem| elem < b);
        if pos < dp.len() {
            dp[pos] = b;
        } else {
            dp.push(b);
        }
    }

    println!("{}", dp.len());
}
