use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        tt: [u32; n],
    }

    let output = (0..n)
        .sorted_unstable_by_key(|&i| tt[i])
        .take(3)
        .map(|i| i + 1)
        .join(" ");
    println!("{output}");
}
