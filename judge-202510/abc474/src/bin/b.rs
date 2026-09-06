use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
    }

    let ans = pp.chunks(10).enumerate().all(|(i, chunk)| {
        let mut sorted = chunk.to_vec();
        sorted.sort_unstable();
        sorted == (10 * i..(10 * (i + 1)).min(n)).collect_vec()
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
