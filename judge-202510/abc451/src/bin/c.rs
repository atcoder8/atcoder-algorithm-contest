use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let queries = (0..q).map(|_| Query::read());
    let mut heap = BinaryHeap::new();
    for query in queries {
        match query {
            Query::Add(h) => heap.push(Reverse(h)),
            Query::Remove(h) => {
                while let Some(Reverse(h_)) = heap.peek()
                    && *h_ <= h
                {
                    heap.pop();
                }
            }
        }

        println!("{}", heap.len());
    }
}

#[derive(Debug, Clone, Copy)]
enum Query {
    Add(u32),
    Remove(u32),
}

impl Query {
    fn read() -> Self {
        input! {
            (qt, h): (u8, u32),
        }

        if qt == 1 {
            Self::Add(h)
        } else {
            Self::Remove(h)
        }
    }
}
