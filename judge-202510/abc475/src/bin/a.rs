use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.chars().join("o");
    println!("{ans}");
}
