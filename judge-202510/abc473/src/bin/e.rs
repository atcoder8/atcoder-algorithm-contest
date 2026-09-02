use std::collections::BTreeSet;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, u32),
        aa: [u32; n],
    }

    let mut acc = vec![0; n + 1];
    for (i, &a) in enumerate(&aa) {
        acc[i + 1] = (acc[i] + a) % k;
    }

    let mut score = 0;
    let mut previous = BTreeSet::<u32>::from_iter([0]);
    for right in 1..=n {
        let prefix_sum = acc[right];
        if previous.contains(&prefix_sum) {
            score += 1;
            previous.clear();
            previous.insert(prefix_sum);
        } else {
            previous.insert(prefix_sum);
        }
    }

    println!("{score}");
}
