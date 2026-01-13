use std::collections::BTreeSet;

use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn replace(n: u32) -> u32 {
    n.to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap().pow(2))
        .sum()
}

fn solve() -> bool {
    input! {
        n: u32,
    }

    let mut pool = BTreeSet::<u32>::new();
    let mut t = n;
    while !pool.contains(&t) {
        pool.insert(t);
        t = replace(t);
    }
    t == 1
}
