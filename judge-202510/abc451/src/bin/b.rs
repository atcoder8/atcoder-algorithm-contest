use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); n],
    }

    let mut counts = vec![[0; 2]; m];
    for &(a, b) in &ab {
        counts[a][0] += 1;
        counts[b][1] += 1;
    }

    let output = counts.iter().map(|pair| pair[1] - pair[0]).join("\n");
    println!("{output}");
}
