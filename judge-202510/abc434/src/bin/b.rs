use itertools::{Itertools, izip};
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, f64); n],
    }

    let mut sum = vec![0.0; m];
    let mut counts = vec![0_usize; m];
    for &(a, b) in &ab {
        sum[a] += b;
        counts[a] += 1;
    }

    let output = izip!(sum, counts)
        .map(|(sum, cnt)| sum / cnt as f64)
        .join("\n");
    println!("{output}");
}
