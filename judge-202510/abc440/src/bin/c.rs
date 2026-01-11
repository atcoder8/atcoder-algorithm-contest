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
        (n, w): (usize, usize),
        mut cc: [u64; n],
    }

    let w2 = 2 * w;

    let ceil_div = (n + w2 - 1) / w2;
    cc.resize(w2 * ceil_div, 0);

    let mut cost = enumerate(&cc)
        .map(|(i, &c)| if i % w2 < w { c } else { 0 })
        .sum::<u64>();
    let mut min_cost = cost;
    for x in 0..w2 {
        for i in 0..ceil_div {
            let sub_cc = &cc[w2 * i..w2 * (i + 1)];
            cost -= sub_cc[x];
            cost += sub_cc[(x + w) % w2];
        }
        min_cost = min_cost.min(cost);
    }

    min_cost
}
