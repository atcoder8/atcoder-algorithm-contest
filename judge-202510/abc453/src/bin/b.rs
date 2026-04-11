use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        (t, x): (usize, u8),
        aa: [u8; t + 1],
    }

    let mut records = vec![(0, aa[0])];
    let mut prev_a = aa[0];
    for (i, &a) in enumerate(&aa).skip(1) {
        if a.abs_diff(prev_a) >= x {
            records.push((i, a));
            prev_a = a;
        }
    }

    let output = records.iter().map(|(t, x)| format!("{t} {x}")).join("\n");
    println!("{output}");
}
