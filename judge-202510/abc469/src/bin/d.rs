use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    println!("{}", solve(n, &ab));
}

fn solve(n: usize, ab: &[(usize, usize)]) -> usize {
    let ab = ab.iter().unique().copied().collect_vec();

    let (a0, b0) = ab[0];

    let mut num_combs = 0;

    for selected in [a0, b0] {
        let mut num_edges = 0;
        let mut degrees = vec![0_usize; n];
        for &(a, b) in &ab {
            if a == selected || b == selected {
                continue;
            }

            num_edges += 1;
            degrees[a] += 1;
            degrees[b] += 1;
        }

        if num_edges == 0 {
            num_combs += n - 1;
        } else {
            num_combs += degrees
                .iter()
                .filter(|&&degree| degree == num_edges)
                .count();
        }
    }
    num_combs -= ab
        .iter()
        .all(|&(a, b)| [a, b].into_iter().any(|v| v == a0 || v == b0)) as usize;

    num_combs
}
