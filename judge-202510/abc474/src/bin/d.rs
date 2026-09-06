use itertools::{Itertools, izip};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64; n],
        bb: [u64; n],
    }

    match solve(&aa, &bb) {
        Some(ww) => println!("Yes\n{}", ww.iter().join(" ")),
        None => println!("No"),
    }
}

fn solve(aa: &[u64], bb: &[u64]) -> Option<Vec<u64>> {
    let pos = izip!(aa, bb).position(|(a, b)| a > b)?;
    let mut ww = vec![1; aa.len()];
    ww[pos] = 10_u64.pow(18);
    Some(ww)
}
