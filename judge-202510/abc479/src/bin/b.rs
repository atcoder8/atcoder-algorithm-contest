use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s
        .chars()
        .dedup_with_count()
        .map(|(cnt, ch)| if ch == 'R' { cnt } else { 0 })
        .max()
        .unwrap();
    println!("{ans}");
}
