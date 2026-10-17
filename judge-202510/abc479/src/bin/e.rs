use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut in_time = vec![0; n];
    let mut out_time = vec![0; n];
    recursive(&graph, None, 0, &mut in_time, &mut out_time, &mut 0);

    let mut imos = vec![0; 2 * n];
    for (i, &(mut a, mut b)) in ab.iter().enumerate() {
        if in_time[a] > in_time[b] {
            std::mem::swap(&mut a, &mut b);
        }

        let mid_period = out_time[b] - in_time[b] + 1;
        if mid_period < n {
            imos[0] += i as i64 + 1;
            imos[2 * n - 1] -= i as i64 + 1;
            imos[in_time[b]] -= i as i64 + 1;
            imos[out_time[b]] += i as i64 + 1;
        } else {
            imos[in_time[b]] += i as i64 + 1;
            imos[out_time[b]] -= i as i64 + 1;
        }
    }
    for i in 0..2 * n - 1 {
        imos[i + 1] += imos[i];
    }

    let output = (0..n).map(|i| imos[in_time[i]]).join(" ");
    println!("{output}");
}

fn recursive(
    graph: &[Vec<usize>],
    parent: Option<usize>,
    current: usize,
    in_time: &mut [usize],
    out_time: &mut [usize],
    time: &mut usize,
) {
    in_time[current] = *time;
    *time += 1;

    graph[current]
        .iter()
        .filter(|&&adjacent| Some(adjacent) != parent)
        .for_each(|&next| recursive(graph, Some(current), next, in_time, out_time, time));

    out_time[current] = *time;
    *time += 1;
}
