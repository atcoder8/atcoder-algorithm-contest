use std::collections::{BTreeSet, VecDeque};

use itertools::{Itertools, chain};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        match solve() {
            Some(vv) => println!("{}\n{}", vv.len(), vv.iter().map(|v| v + 1).join(" ")),
            None => println!("-1"),
        }
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &ab {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut distances = vec![[None::<usize>; 2]; n];
    let mut queue = VecDeque::from_iter([(0, 0)]);
    let mut start = None::<usize>;
    while let Some((curr, dist)) = queue.pop_front() {
        let parity = dist % 2;

        if distances[curr][parity].is_some() {
            continue;
        }

        distances[curr][parity] = Some(dist);

        if distances[curr][1 - parity].is_some() {
            start = Some(curr);
            break;
        }

        queue.extend(graph[curr].iter().map(|&next| (next, dist + 1)));
    }

    let start = start?;

    let find_path = |init_parity: usize| {
        let mut path = vec![start];
        let mut curr = start;
        for dist in (0..distances[start][init_parity].unwrap()).rev() {
            let next = *graph[curr]
                .iter()
                .find(|&&adjacent| distances[adjacent][dist % 2] == Some(dist))
                .unwrap();
            path.push(next);
            curr = next;
        }

        path
    };

    let path1 = find_path(0);
    let path2 = find_path(1);

    let nodes = BTreeSet::from_iter(path1.iter().copied());

    let first_common = *path2[1..].iter().find(|node| nodes.contains(node)).unwrap();

    let path1 = path1
        .into_iter()
        .take_while_inclusive(|&node| node != first_common);
    let path2 = path2
        .into_iter()
        .skip(1)
        .take_while(|&node| node != first_common)
        .collect_vec();
    let path = chain(path1, path2.into_iter().rev()).collect();
    Some(path)
}
