use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        cs: [(Usize1, i32); n],
    }

    let mut max_sizes = vec![-1; m];
    for &(c, s) in &cs {
        max_sizes[c] = max_sizes[c].max(s);
    }

    let output = max_sizes.iter().join(" ");
    println!("{output}");
}
