use std::collections::BinaryHeap;

use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        (h, w, n): (u64, u64, usize),
        hw: [(u64, u64); n],
    }

    let mut heap1 = BinaryHeap::from_iter(enumerate(&hw).map(|(i, &(h, w))| (h, w, i)));
    let mut heap2 = BinaryHeap::from_iter(enumerate(&hw).map(|(i, &(h, w))| (w, h, i)));
    let mut positions = vec![None::<(u64, u64)>; n];
    let mut offset_h = 0;
    let mut offset_w = 0;
    for _ in 0..n {
        let pop_used_heap = |heap: &mut BinaryHeap<(u64, u64, usize)>| {
            while heap.peek().is_some_and(|&(_, _, i)| positions[i].is_some()) {
                heap.pop();
            }
        };

        pop_used_heap(&mut heap1);
        pop_used_heap(&mut heap2);

        let (h1, w1, i1) = *heap1.peek().unwrap();
        let (w2, h2, i2) = *heap2.peek().unwrap();

        if h1 == h - offset_h {
            positions[i1] = Some((offset_h, offset_w));
            offset_w += w1;
        } else {
            assert_eq!(w - offset_w, w2);
            positions[i2] = Some((offset_h, offset_w));
            offset_h += h2;
        }
    }

    let output = positions
        .iter()
        .map(|pos| {
            let (offset_h, offset_w) = pos.unwrap();
            format!("{} {}", offset_h + 1, offset_w + 1)
        })
        .join("\n");
    println!("{output}");
}
