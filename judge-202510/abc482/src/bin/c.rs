use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (_h, _w, n, q): (usize, usize, usize, usize),
        rc: [(Usize1, Usize1); n],
        queries: [(u8, Usize1, usize); q],
    }

    let mut horizontal = vec![];
    let mut vertical = vec![];
    for &(r, c) in &rc {
        horizontal.push(r);
        vertical.push(c);
    }
    horizontal.sort_unstable();
    vertical.sort_unstable();

    let solve = |qt: u8, left: usize, right: usize| {
        let v = if qt == 1 { &horizontal } else { &vertical };
        let lower = v.lower_bound(&left);
        let upper = v.lower_bound(&right);
        upper - lower
    };

    let output = queries
        .iter()
        .map(|&(qt, left, right)| solve(qt, left, right))
        .join("\n");
    println!("{output}");
}
