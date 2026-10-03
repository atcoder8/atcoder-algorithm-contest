use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let output = (0..n).map(|i| m / n + (i < m % n) as usize).join("\n");
    println!("{output}");
}
