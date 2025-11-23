use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .dedup_with_count()
        .tuple_windows()
        .map(
            |((cnt1, d1), (cnt2, d2))| {
                if d1 + 1 == d2 { cnt1.min(cnt2) } else { 0 }
            },
        )
        .sum::<usize>();
    println!("{ans}");
}
