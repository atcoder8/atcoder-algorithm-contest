use itertools::{Itertools, enumerate, iproduct};
use proconio::input;

const SENTINEL: u64 = 10_u64.pow(18) + 1;

fn main() {
    input! {
        (n, k): (usize, usize),
        ccc: [[u64; n]; n],
    }

    let mut doubling = vec![ccc];
    for exp in (0..).take_while(|&exp| k >> exp != 0) {
        let curr = &doubling[exp];
        let mut next = vec![vec![SENTINEL; n]; n];
        for (from, mid, to) in iproduct!(0..n, 0..n, 0..n) {
            next[from][to] = next[from][to].min(curr[from][mid] + curr[mid][to]);
        }
        doubling.push(next);
    }

    let mut costs = vec![vec![SENTINEL; n]; n];
    (0..n).for_each(|i| costs[i][i] = 0);
    for (exp, transition) in enumerate(&doubling) {
        if k >> exp & 1 == 1 {
            let mut next_costs = vec![vec![SENTINEL; n]; n];
            for (from, mid, to) in iproduct!(0..n, 0..n, 0..n) {
                next_costs[from][to] =
                    next_costs[from][to].min(costs[from][mid] + transition[mid][to]);
            }
            costs = next_costs;
        }
    }

    let output = (0..n).map(|s| costs[s][s]).join("\n");
    println!("{output}");
}
