use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uvc: [(Usize1, Usize1, u32); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, color) in &uvc {
        graph[u].push((v, color));
    }

    // 最大2色について最小コストを格納
    let mut min_costs: Vec<Vec<(usize, u32)>> = vec![vec![]; n];
    let mut queue = VecDeque::from([(0_usize, 0_usize, (m + 1) as u32)]);
    while let Some((curr, cost, color)) = queue.pop_front() {
        let min_cost = &mut min_costs[curr];

        if min_cost.len() >= 2 || min_cost.iter().any(|v| v.1 == color) {
            continue;
        }

        min_cost.push((cost, color));

        queue.extend(
            graph[curr]
                .iter()
                .filter(|v| v.1 != color)
                .map(|&(next, next_color)| (next, cost + 1, next_color)),
        );
    }

    let min_cost = min_costs[n - 1].iter().map(|v| v.0).min();
    match min_cost {
        Some(min_cost) => println!("{min_cost}"),
        None => println!("-1"),
    }
}
