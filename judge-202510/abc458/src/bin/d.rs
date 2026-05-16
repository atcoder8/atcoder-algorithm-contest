use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u32,
        q: usize,
        ab: [(u32, u32); q],
    }

    let mut small = BinaryHeap::<u32>::from([x]);
    let mut large = BinaryHeap::<Reverse<u32>>::new();
    for &(a, b) in &ab {
        let median = *small.peek().unwrap();
        for v in [a, b] {
            if v <= median {
                small.push(v);
            } else {
                large.push(Reverse(v));
            }
        }

        while small.len() > large.len() + 1 {
            large.push(Reverse(small.pop().unwrap()));
        }

        while small.len() < large.len() + 1 {
            small.push(large.pop().unwrap().0);
        }

        println!("{}", small.peek().unwrap());
    }
}
