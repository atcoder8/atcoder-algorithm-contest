use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (q, v): (usize, i64),
    }

    let queries = (0..q).map(|_| Query::read());

    let mut heap = BinaryHeap::new();
    for query in queries {
        match query {
            Query::Plug { t, w } => heap.push(w - t),
            Query::Unplug { t } => {
                let level = if let Some(w) = heap.pop() {
                    (w + t).min(v)
                } else {
                    -1
                };
                println!("{level}");
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Plug { t: i64, w: i64 },
    Unplug { t: i64 },
}

impl Query {
    fn read() -> Self {
        input! {
            (qt, t): (u8, i64),
        }

        if qt == 1 {
            input! {
                w: i64,
            }

            Self::Plug { t, w }
        } else {
            Self::Unplug { t }
        }
    }
}
