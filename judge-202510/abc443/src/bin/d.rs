use itertools::{Itertools, izip};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        n: usize,
        rr: [Usize1; n],
    }

    let mut ss = rr.clone();
    for i in 1..n {
        ss[i] = ss[i].min(ss[i - 1] + 1);
    }
    for i in (0..n - 1).rev() {
        ss[i] = ss[i].min(ss[i + 1] + 1);
    }

    izip!(&rr, &ss).map(|(r, s)| r - s).sum::<usize>()
}
