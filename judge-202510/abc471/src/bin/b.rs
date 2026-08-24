use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let ans = ss
        .into_iter()
        .map(|s| s.to_lowercase())
        .sorted_unstable()
        .dedup_with_count()
        .map(|v| v.0)
        .max()
        .unwrap();
    println!("{ans}");
}
