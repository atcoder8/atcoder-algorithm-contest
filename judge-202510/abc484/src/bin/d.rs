use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut num_combs = 1_usize;
    let mut counts = vec![0_usize; n];
    let mut pos = 0;
    for (cnt, &a) in aa.iter().dedup_with_count() {
        num_combs += pos - counts[a];
        counts[a] += cnt;
        pos += cnt;
    }

    println!("{num_combs}");
}
