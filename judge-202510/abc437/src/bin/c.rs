use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        n: usize,
        mut wp: [(i64, i64); n],
    }

    let sum_w = wp.iter().map(|&(w, _)| w).sum::<i64>();
    let mut acc = 0;
    n - wp
        .iter()
        .sorted_unstable_by_key(|&(w, p)| Reverse(w + p))
        .take_while(|&(w, p)| {
            if acc >= sum_w {
                return false;
            }

            acc += w + p;
            true
        })
        .count()
}
