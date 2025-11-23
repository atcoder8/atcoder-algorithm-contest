use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let solve = |i: usize| (0..i).rev().find(|&pos| aa[pos] > aa[i]);
    let output = (0..n)
        .map(|i| match solve(i) {
            Some(pos) => (pos + 1).to_string(),
            None => "-1".to_string(),
        })
        .join("\n");
    println!("{output}");
}
