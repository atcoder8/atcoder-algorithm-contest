use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        q: usize,
        s: String,
        t: String,
        lr: [(Usize1, usize); q],
    }

    let positions = if s.len() >= t.len() {
        (0..=s.len() - t.len())
            .filter(|&left| s[left..left + t.len()] == t)
            .collect_vec()
    } else {
        vec![]
    };

    let is_ok = |l: usize, r: usize| {
        let index = positions.lower_bound(&l);
        index < positions.len() && positions[index] + t.len() <= r
    };

    let output = lr
        .iter()
        .map(|&(l, r)| if is_ok(l, r) { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}
