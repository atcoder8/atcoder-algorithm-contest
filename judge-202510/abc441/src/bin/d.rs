use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, l, s, t): (usize, usize, usize, u64, u64),
        uvc: [(Usize1, Usize1, u64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvc {
        graph[u].push((v, weight));
    }

    let mut vv = vec![];

    let mut stack = vec![(0_usize, 0_usize, 0_u64)];
    while let Some((current, dist, cost)) = stack.pop() {
        if dist == l {
            if s <= cost && cost <= t {
                vv.push(current);
            }
            continue;
        }

        stack.extend(
            graph[current]
                .iter()
                .map(|&(adjacent, weight)| (adjacent, dist + 1, cost + weight)),
        );
    }

    vv.sort_unstable();
    vv.dedup();

    let output = vv.iter().map(|v| v + 1).join(" ");
    println!("{output}");
}
