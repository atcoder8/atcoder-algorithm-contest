// unfinished

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let mut inclusive = 0_usize;
    let mut exclusive = 0_usize;
    for bits in 0_u32..1 << m {
        if bits.count_ones() > 2 {
            continue;
        }

        let tt = ss
            .iter()
            .map(|s| {
                let mut t = s.clone();
                for i in 0..m {
                    if bits >> i & 1 == 1 {
                        t[i] = '*';
                    }
                }

                t
            })
            .sorted_unstable();
        let num = tt
            .dedup_with_count()
            .map(|(cnt, _)| cnt * (cnt - 1) / 2)
            .sum::<usize>();
        if bits.count_ones() % 2 == 0 {
            inclusive += num;
        } else {
            exclusive += num;
        }
    }

    println!("{}", inclusive - exclusive);
}
