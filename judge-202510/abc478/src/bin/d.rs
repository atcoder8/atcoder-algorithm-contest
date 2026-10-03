use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        lrx: [(Usize1, usize, Usize1); q],
    }

    let events = lrx
        .iter()
        .flat_map(|&(l, r, x)| [(l, x, true), (r, x, false)])
        .sorted_unstable_by_key(|v| v.0)
        .collect_vec();

    let mut sizes = vec![0_usize; n];
    let mut size = 0_usize;
    let mut counts = vec![0_usize; q + 1];
    let mut cursor = 0;
    for i in 0..n {
        while cursor < events.len() && events[cursor].0 == i {
            let event = events[cursor];

            if event.2 {
                counts[event.1] += 1;
                if counts[event.1] == 1 {
                    size += 1;
                }
            } else {
                counts[event.1] -= 1;
                if counts[event.1] == 0 {
                    size -= 1;
                }
            }

            cursor += 1;
        }

        sizes[i] = size;
    }

    let output = sizes.iter().join(" ");
    println!("{output}");
}
