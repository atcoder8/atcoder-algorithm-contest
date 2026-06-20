use std::cmp::Reverse;

use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut hl: [(u32, u32); n],
        q: usize,
        tt: [u32; q],
    }

    let it = enumerate(tt).sorted_unstable_by_key(|v| Reverse(v.1));
    hl.sort_unstable_by_key(|v| Reverse(v.1));
    let mut idx = 0;

    let mut answers = vec![0; q];
    let mut max = 0;
    for (i, t) in it {
        while idx < n && hl[idx].1 > t {
            max = max.max(hl[idx].0);
            idx += 1;
        }
        answers[i] = max;
    }

    let output = answers.iter().join("\n");
    println!("{output}");
}
