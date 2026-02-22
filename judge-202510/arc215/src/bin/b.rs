use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| {
            let cc = solve();
            format!("{}\n{}", cc.len(), cc.iter().map(|c| c).join(" "))
        })
        .join("\n");
    println!("{output}");
}

fn solve() -> Vec<usize> {
    input! {
        n: usize,
        aa: [Usize1; 2 * n],
    }

    let mut cc = vec![];
    let mut collected = vec![false; n];
    let mut disposed = vec![false; n];
    let mut is_collecting = true;
    for (i, &a) in enumerate(&aa) {
        if (is_collecting && collected[a]) || (!is_collecting && disposed[a]) {
            cc.push(i);
            is_collecting = !is_collecting;
        }
        [&mut disposed, &mut collected][is_collecting as usize][a] = true;
    }

    cc
}
