use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
        w: usize,
        ss: [Chars; n],
    }

    let mut graph = (0..n).map(|i| vec![i]).collect_vec();
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![vec![false; w]; n];
    let mut passed = vec![vec![false; w]; n];
    for start_city in 0..n {
        if visited[start_city][0] {
            continue;
        }

        let mut stack = vec![(start_city, 0, true)];
        while let Some((city, date, forward)) = stack.pop() {
            if !forward {
                passed[city][date] = false;
                continue;
            }

            if passed[city][date] {
                return true;
            }

            if ss[city][date] == 'x' || visited[city][date] {
                continue;
            }

            visited[city][date] = true;
            passed[city][date] = true;

            stack.push((city, date, false));
            stack.extend(
                graph[city]
                    .iter()
                    .map(|&next_city| (next_city, (date + 1) % w, true)),
            );
        }
    }

    false
}
