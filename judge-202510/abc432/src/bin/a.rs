use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut seq: [u8; 3],
    }

    seq.sort_unstable_by_key(|&v| Reverse(v));

    println!("{}", seq.iter().join(""));
}
