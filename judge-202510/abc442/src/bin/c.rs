use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut degrees = vec![1_usize; n];
    for &(a, b) in &ab {
        degrees[a] += 1;
        degrees[b] += 1;
    }

    let solve = |i: usize| {
        let rem = n - degrees[i];
        rem * rem.saturating_sub(1) * rem.saturating_sub(2) / 6
    };

    let output = (0..n).map(solve).join(" ");
    println!("{output}");
}
