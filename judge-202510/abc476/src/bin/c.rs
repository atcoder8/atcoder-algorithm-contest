use std::collections::BTreeSet;

use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let mut s = BTreeSet::from_iter(enumerate(&aa[..2]).map(|(i, &a)| (a, i)));
    let output = (2..n)
        .map(|i| {
            s.insert((aa[i], i));
            s.iter().nth_back(2).unwrap().0
        })
        .join("\n");
    println!("{output}");
}
