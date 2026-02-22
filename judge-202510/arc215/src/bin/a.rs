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
        (n, k, l): (usize, usize, u64),
        mut aa: [u64; n],
    }

    aa.sort_unstable();

    let intervals = aa
        .iter()
        .tuple_windows()
        .map(|(a1, a2)| a2 - a1)
        .sorted_unstable_by_key(|&interval| Reverse(interval));

    let calc_paddings =
        |shrink_dist: u64| (aa[0] + shrink_dist / 2, l - (aa[n - 1] - shrink_dist / 2));

    let calc_score = |shrink_dist: u64, num_shrinks: usize| {
        let (first_padding, last_padding) = calc_paddings(shrink_dist);
        let mut score = shrink_dist / 2;
        let rem = k - num_shrinks;
        if rem > 0 {
            score +=
                first_padding.max(last_padding) + (first_padding + last_padding) * (rem as u64 - 1);
        }
        score
    };

    let mut max_score = calc_score(0, 0);
    let mut shrink_dist = 0;
    for (i, interval) in enumerate(intervals).take(k) {
        shrink_dist += interval;
        max_score = max_score.max(calc_score(shrink_dist, i + 1));
    }

    max_score
}
