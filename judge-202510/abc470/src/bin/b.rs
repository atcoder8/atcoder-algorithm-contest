use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        cc: [u8; n],
    }

    let max = cc
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .map(|v| v.0)
        .max()
        .unwrap();
    println!("{}", n - max);
}
