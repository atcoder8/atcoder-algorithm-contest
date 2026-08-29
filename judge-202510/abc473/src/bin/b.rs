use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let sum = aa
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .map(|(cnt, &a)| a * (cnt % 2) as u32)
        .sum::<u32>();
    println!("{sum}");
}
