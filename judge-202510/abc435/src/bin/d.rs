use argio::proconio::{input, marker::Usize1};
use proconio::fastout;

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        xy: [(Usize1, Usize1); m],
        q: usize,
        queries: [(u8, Usize1); q],
    }

    let mut inv_graph = vec![vec![]; n];
    for &(u, v) in &xy {
        inv_graph[v].push(u);
    }

    let mut reachable = vec![false; n];

    for &(qt, v) in &queries {
        if qt == 2 {
            println!("{}", if reachable[v] { "Yes" } else { "No" });
            continue;
        }

        let mut stack = vec![v];
        while let Some(curr) = stack.pop() {
            if reachable[curr] {
                continue;
            }

            reachable[curr] = true;
            stack.extend(inv_graph[curr].iter().cloned());
        }
    }
}
