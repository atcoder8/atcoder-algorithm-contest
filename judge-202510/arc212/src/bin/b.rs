use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        xyc: [(Usize1, Usize1, u64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &xyc {
        graph[u].push((v, weight));
    }

    let mut sum_costs = vec![None::<u64>; n];
    let (x0, y0, c0) = xyc[0];
    let mut heap = BinaryHeap::from_iter([Reverse((c0, y0))]);
    while let Some(Reverse((sum_cost, current))) = heap.pop() {
        if sum_costs[current].is_some() {
            continue;
        }

        sum_costs[current] = Some(sum_cost);

        heap.extend(
            graph[current]
                .iter()
                .map(|&(adjacent, cost)| Reverse((sum_cost + cost, adjacent))),
        );
    }

    let sum_cost = sum_costs[x0];
    match sum_cost {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
