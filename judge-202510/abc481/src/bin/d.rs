use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, s, t): (usize, usize, Usize1, Usize1),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
    }
    graph.iter_mut().for_each(|edges| edges.sort_unstable());

    let mut distances = vec![None::<u32>; n];
    let mut queue = VecDeque::from([(s, 0)]);
    while let Some((curr, dist)) = queue.pop_front() {
        if distances[curr].is_some() {
            continue;
        }

        distances[curr] = Some(dist);

        let edges = &graph[curr];
        if !edges.is_empty() {
            queue.push_front((edges[0], dist));
            queue.extend(edges[1..].iter().map(|&adjacent| (adjacent, dist + 1)));
        }
    }

    match distances[t] {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
