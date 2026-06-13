use itertools::{Itertools, enumerate};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        aaa: [[Usize1]; n],
    }

    let mut gifts = vec![vec![]; n];
    for (i, aa) in enumerate(&aaa) {
        for &a in aa {
            gifts[a].push(i);
        }
    }
    gifts.iter_mut().for_each(|v| v.sort_unstable());

    for persons in &gifts {
        println!(
            "{} {}",
            persons.len(),
            persons.iter().map(|i| i + 1).join(" ")
        );
    }
}
