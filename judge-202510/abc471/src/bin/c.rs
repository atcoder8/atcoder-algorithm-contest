use std::cmp::Reverse;

use itertools::chain;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut small = vec![];
    let mut large = vec![];
    for &a in &aa {
        [&mut small, &mut large][(a >= 0) as usize].push(a);
    }
    small.sort_unstable();
    large.sort_unstable_by_key(|&v| Reverse(v));

    let mut total = 0;
    let mut x = 0;
    for _ in 0..n {
        let next_x = *chain(small.last(), large.last())
            .min_by_key(|candidate| candidate.abs_diff(x))
            .unwrap();

        total += next_x.abs_diff(x);

        [&mut small, &mut large][(next_x >= 0) as usize].pop();
        x = next_x;
    }

    println!("{total}");
}
