use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, y): (usize, usize, u64),
        uvt: [(Usize1, Usize1, u64); m],
        xx: [u64; n],
    }

    let mut graph = vec![vec![]; n + 2];
    for &(u, v, weight) in &uvt {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }
    graph[n].push((n + 1, y));
    for (i, &x) in enumerate(&xx) {
        graph[i].push((n, x));
        graph[n + 1].push((i, x));
    }

    let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
    let mut distances = vec![None::<u64>; n + 2];
    while let Some((Reverse(dist), curr)) = heap.pop() {
        if distances[curr].is_some() {
            continue;
        }

        distances[curr] = Some(dist);

        heap.extend(
            graph[curr]
                .iter()
                .map(|&(adjacent, weight)| (Reverse(dist + weight), adjacent)),
        );
    }

    let output = distances[1..n].iter().map(|dist| dist.unwrap()).join(" ");
    println!("{output}");
}
