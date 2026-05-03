// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Alice" } else { "Bob" })
        .join("\n");
    println!("{output}");
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ca = aa
        .iter()
        .copied()
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();
    ca.insert(0, (0, 0));
    ca.reverse();

    let calc_grundy = |c: usize, diff: usize| match diff {
        0 => c,
        1 => 0,
        _ => {
            if diff == 1 {
                c
            } else {
                1
            }
        }
    };

    let mut grundy = false;
    for (&(c, a), &(_, prev_a)) in ca.iter().tuple_windows() {
        let diff = a - prev_a;
        grundy ^= calc_grundy(c, diff) != 0;
    }
    grundy
}
