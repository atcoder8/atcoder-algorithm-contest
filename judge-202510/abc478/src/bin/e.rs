use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

use crate::scc::SCC;

fn main() {
    match solve() {
        Some(aa) => println!("Yes\n{}", aa.iter().map(|a| a + 1).join(" ")),
        None => println!("No"),
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        (n, q): (usize, usize),
        tuv: [(u8, Usize1, Usize1); q],
    }

    let mut scc_graph = SCC::new(n);
    for &(_, u, v) in &tuv {
        scc_graph.add_edge(u, v);
    }

    let scc = scc_graph.scc();

    let mut aa = vec![0; n];
    for (i, component) in enumerate(&scc) {
        component.iter().for_each(|&u| aa[u] = i);
    }

    if tuv.iter().any(|&(t, u, v)| t == 1 && aa[u] >= aa[v]) {
        return None;
    }

    Some(aa)
}

pub mod scc {
    #[derive(Debug, Clone)]
    pub struct SCC {
        graph: Vec<Vec<usize>>,
        inv_graph: Vec<Vec<usize>>,
    }

    impl SCC {
        pub fn new(n: usize) -> Self {
            Self {
                graph: vec![vec![]; n],
                inv_graph: vec![vec![]; n],
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.graph[from].push(to);
            self.inv_graph[to].push(from);
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            let n = self.graph.len();

            let mut order = vec![];
            let mut visited = vec![false; n];
            for start_node in 0..n {
                if !visited[start_node] {
                    order.append(&mut post_order_traversal(
                        &self.graph,
                        &mut visited,
                        start_node,
                    ));
                }
            }

            let mut scc = vec![];
            let mut visited = vec![false; n];
            for &start_node in order.iter().rev() {
                if !visited[start_node] {
                    scc.push(post_order_traversal(
                        &self.inv_graph,
                        &mut visited,
                        start_node,
                    ));
                }
            }

            scc
        }
    }

    fn post_order_traversal(
        graph: &[Vec<usize>],
        visited: &mut [bool],
        start_node: usize,
    ) -> Vec<usize> {
        let mut post_order = vec![];

        let mut stack = vec![(start_node, false)];

        while let Some((node, back)) = stack.pop() {
            if back {
                post_order.push(node);
            }

            if visited[node] {
                continue;
            }

            visited[node] = true;

            stack.push((node, true));

            stack.extend(graph[node].iter().map(|&next_node| (next_node, false)));
        }

        post_order
    }
}
