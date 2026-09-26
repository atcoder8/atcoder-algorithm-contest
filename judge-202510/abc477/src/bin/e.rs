use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [u64; n],
        bb: [u64; n],
        st: [(Usize1, Usize1); q],
    }

    // 外周の距離の累積和
    let mut acc = vec![0; n + 1];
    for (i, &a) in enumerate(&aa) {
        acc[i + 1] = acc[i] + a;
    }

    // 外周の左回りと右回りを試す
    let calc_perimeter_dist =
        |s: usize, t: usize| (acc[t] - acc[s]).min(acc[n] - (acc[t] - acc[s]));

    // ダイクストラ法のために重み付きグラフを作成
    let mut graph = vec![vec![]; n + 1];
    for (i, &a) in enumerate(&aa) {
        graph[i].push(((i + 1) % n, a));
        graph[(i + 1) % n].push((i, a));
    }
    for (i, &b) in enumerate(&bb) {
        graph[i].push((n, b));
        graph[n].push((i, b));
    }

    // ダイクストラ法により中央を始点とした距離を求める
    let mut dist_from_center = vec![u64::MAX; n + 1];
    let mut heap = BinaryHeap::from_iter([(Reverse(0), n)]);
    while let Some((Reverse(dist), curr)) = heap.pop() {
        if dist_from_center[curr] < u64::MAX {
            continue;
        }

        dist_from_center[curr] = dist;

        heap.extend(
            graph[curr]
                .iter()
                .map(|&(adjacent, weight)| (Reverse(dist + weight), adjacent)),
        );
    }

    let solve = |s: usize, t: usize| {
        if t == n {
            // 終点が中央の場合はダイクストラ法の結果がそのまま答えになる
            dist_from_center[s]
        } else {
            // 始点と終点が共に外周にある場合は外周のみを通る場合と中央を通る場合を試す
            calc_perimeter_dist(s, t).min(dist_from_center[s] + dist_from_center[t])
        }
    };

    let output = st.iter().map(|&(s, t)| solve(s, t)).join("\n");
    println!("{output}");
}
