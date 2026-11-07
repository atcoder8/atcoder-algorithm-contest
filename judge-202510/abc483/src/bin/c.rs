use itertools::{Itertools, chain};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut transitions = vec![vec![]; n];
    for &(a, b) in &ab {
        transitions[a].push(b);
    }

    let first = (0..n).rev().max_by_key(|&i| transitions[i].len()).unwrap();
    let second = (0..n)
        .rev()
        .filter(|&i| i != first)
        .max_by_key(|&i| transitions[i].len())
        .unwrap();

    let solve = |k: usize, counts: &mut [usize]| {
        transitions[k].iter().for_each(|&v| counts[v] += 1);

        let mut candidates =
            chain!([first, second], transitions[k].iter().copied()).filter(|&i| i != k);
        let mut winner = candidates.next().unwrap();
        for candidate in candidates {
            if counts[candidate] > counts[winner]
                || (counts[candidate] == counts[winner] && candidate < winner)
            {
                winner = candidate;
            }
        }

        transitions[k].iter().for_each(|&v| counts[v] -= 1);

        winner
    };

    let mut counts = transitions.iter().map(|v| v.len()).collect_vec();

    let output = (0..n).map(|k| solve(k, &mut counts) + 1).join("\n");
    println!("{output}");
}
