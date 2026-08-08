use std::collections::BTreeMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (_n, q): (usize, usize),
    }

    let queries = (0..q).map(|_| Query::read());

    let mut xor = 0;
    let mut counts = BTreeMap::<usize, u32>::new();
    for query in queries {
        match query {
            Query::Increment(x) => {
                let cnt = counts.entry(x).or_default();
                *cnt += 1;
                xor ^= (*cnt - 1) ^ *cnt;
            }
            Query::Truncate => {
                counts.values_mut().for_each(|cnt| {
                    *cnt -= 1;
                    xor ^= (*cnt + 1) ^ *cnt;
                });
                counts.retain(|_, cnt| *cnt > 0);
            }
        }

        println!("{xor}");
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Increment(usize),
    Truncate,
}

impl Query {
    fn read() -> Self {
        input! {
            qt: u8,
        }

        if qt == 1 {
            input! {
                x: usize,
            }

            Self::Increment(x)
        } else {
            Self::Truncate
        }
    }
}
