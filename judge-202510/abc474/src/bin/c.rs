use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        mut pp: [Usize1; n],
        mut aa: [Usize1; q],
    }

    let mut unique = pp.into_iter().chain(aa).rev().unique().collect_vec();
    unique.reverse();
    println!("{}", unique.iter().map(|p| p + 1).join(" "));
}
