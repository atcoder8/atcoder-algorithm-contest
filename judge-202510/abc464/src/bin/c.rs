use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        mut adb: [(Usize1, Usize1, Usize1); n],
    }

    adb.sort_unstable_by_key(|v| v.1);

    let mut num_types = 0_usize;
    let mut counts = vec![0_usize; n];

    for &(a, _, _) in &adb {
        counts[a] += 1;
        if counts[a] == 1 {
            num_types += 1;
        }
    }

    let mut idx = 0;
    let output = (0..m)
        .map(|day| {
            while idx < n && adb[idx].1 <= day {
                let (a, _, b) = adb[idx];
                counts[a] -= 1;
                if counts[a] == 0 {
                    num_types -= 1;
                }
                counts[b] += 1;
                if counts[b] == 1 {
                    num_types += 1;
                }
                idx += 1;
            }

            num_types
        })
        .join("\n");
    println!("{output}");
}
