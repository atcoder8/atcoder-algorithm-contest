use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

const MAX_EXP: usize = 30;

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [Usize1; n],
        tb: [(usize, Usize1); q],
    }

    let transitions = enumerate(&aa)
        .map(|(i, &a)| Transition {
            destination: a,
            add_quantity: (i + 1) as u64,
        })
        .collect_vec();
    let mut double_transitions = vec![transitions];
    for exp in 0..MAX_EXP {
        let curr_transitions = &double_transitions[exp];
        let next_transitions = (0..n)
            .map(|i| Transition {
                destination: curr_transitions[curr_transitions[i].destination].destination,
                add_quantity: curr_transitions[i].add_quantity
                    + curr_transitions[curr_transitions[i].destination].add_quantity,
            })
            .collect();
        double_transitions.push(next_transitions);
    }

    let solve = |t: usize, b: usize| {
        let mut curr_dest = b;
        let mut sum_quantity = 0;
        for (exp, transitions) in enumerate(&double_transitions) {
            if t >> exp & 1 == 1 {
                let transition = transitions[curr_dest];
                curr_dest = transition.destination;
                sum_quantity += transition.add_quantity;
            }
        }

        sum_quantity
    };

    let output = tb.iter().map(|&(t, b)| solve(t, b)).join("\n");
    println!("{output}");
}

#[derive(Debug, Clone, Copy)]
struct Transition {
    destination: usize,
    add_quantity: u64,
}
