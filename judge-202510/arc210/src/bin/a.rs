// unfinished

use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        ix: [(Usize1, usize); q],
    }

    let mut sum_cost = n * (n + 1) / 2;
    let mut stairs = BTreeMap::<usize, usize>::new();
    stairs.insert(n, 0);
    for &(i, x) in &ix {
        let added = stairs
            .range(..=i)
            .next_back()
            .map(|(_, &value)| value)
            .unwrap_or(0)
            + x;
        *stairs.entry(i).or_default() = added;
        let mut prev = i + 1;
        loop {
            match stairs.range(prev..).next() {
                Some((&key, &value)) if value < added => {
                    sum_cost += (added - value) * (key - prev);
                    stairs.remove(&key);
                    prev = key;
                }
                _ => break,
            }
        }
        stairs.insert(prev, added);
    }

    println!("{sum_cost}");
}
