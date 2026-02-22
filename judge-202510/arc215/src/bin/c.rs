use std::cmp::Reverse;

use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        n: usize,
        xyz: [(Usize1, Usize1, Usize1); n],
    }

    let surnames = enumerate(&xyz)
        .map(|(i, &(x, y, z))| Surname {
            id: i,
            params: [x, y, z],
        })
        .collect_vec();
    let mut sorted_surnames = [surnames.clone(), surnames.clone(), surnames];
    enumerate(&mut sorted_surnames).for_each(|(i, surnames)| {
        surnames.sort_unstable_by_key(|surname| Reverse(surname.params[i]))
    });

    let mut possibles = vec![false; n];
    let mut progresses = [0; 3];
    let mut boundaries = [0; 3];
    for (axis, surname) in enumerate(&sorted_surnames) {
        boundaries[axis] = surname[0].params[axis];
    }
    loop {
        let Some(axis) = (0..3).find(|&axis| {
            progresses[axis] < n
                && sorted_surnames[axis][progresses[axis]].params[axis] >= boundaries[axis]
        }) else {
            break;
        };

        let surname = sorted_surnames[axis][progresses[axis]];
        possibles[surname.id] = true;
        for (i, v) in enumerate(surname.params) {
            boundaries[i] = boundaries[i].min(v);
        }
        progresses[axis] += 1;
    }

    possibles.iter().filter(|&&possible| possible).count()
}

#[derive(Debug, Clone, Copy)]
struct Surname {
    id: usize,
    params: [usize; 3],
}
