use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let ans = n.chars().all_equal();
    println!("{}", if ans { "Yes" } else { "No" });
}
