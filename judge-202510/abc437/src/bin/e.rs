use std::collections::BTreeMap;

use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, u32); n],
    }

    let mut graph = vec![BTreeMap::<u32, Vec<usize>>::new(); n + 1];
    for (i, &(x, y)) in enumerate(&xy) {
        graph[x].entry(y).or_default().push(i + 1);
    }

    let mut pp = vec![];
    let mut stack = vec![vec![0]];
    while let Some(currents) = stack.pop() {
        pp.extend_from_slice(&currents);
        let mut merged = BTreeMap::<u32, Vec<usize>>::new();
        for (&y, next) in currents.iter().flat_map(|&current| &graph[current]) {
            let values = merged.entry(y).or_default();
            values.extend(next);
        }
        merged
            .values_mut()
            .for_each(|values| values.sort_unstable());
        stack.extend(merged.into_iter().map(|(_, next)| next).rev());
    }

    println!("{}", pp[1..].iter().join(" "));
}
