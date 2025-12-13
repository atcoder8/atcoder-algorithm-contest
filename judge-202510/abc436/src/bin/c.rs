use std::collections::BTreeSet;

use itertools::{Itertools, iproduct};
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        (_n, m): (usize, usize),
        rc: [(Usize1, Usize1); m],
    }

    let mut placed = BTreeSet::new();
    let mut cnt = 0_usize;
    for &(r, c) in &rc {
        let coords = iproduct!(0..2, 0..2)
            .map(|(dr, dc)| (r + dr, c + dc))
            .collect_vec();
        if coords.iter().all(|&coord| !placed.contains(&coord)) {
            placed.extend(coords);
            cnt += 1;
        }
    }
    println!("{cnt}");
}
