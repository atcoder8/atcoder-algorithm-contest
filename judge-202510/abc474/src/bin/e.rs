use std::cmp::Reverse;

use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> u64 {
    input! {
        n: usize,
        mut ab: [(u64, u64); n],
    }

    let min_a = ab.iter().map(|v| v.0).min().unwrap();
    let sum_a = ab.iter().map(|v| v.0).sum::<u64>();

    ab.sort_unstable_by_key(|(a, b)| Reverse(a - b));

    let mut acc_diff = vec![0; n + 1];
    for (i, &(a, b)) in enumerate(&ab) {
        acc_diff[i + 1] = acc_diff[i] + a - b;
    }

    enumerate(acc_diff)
        .map(|(k, prefix_sum)| sum_a - prefix_sum + min_a * (2 * k).saturating_sub(n) as u64)
        .min()
        .unwrap()
}
