use std::borrow::Cow;
use std::collections::BTreeMap;

use itertools::{Itertools, enumerate, iproduct};
use ndarray::prelude::*;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| match solve() {
            Some(aaa) => {
                let formatted_array = aaa
                    .map(|a| a + 1)
                    .axis_iter(Axis(0))
                    .map(|aa| aa.iter().join(" "))
                    .join("\n");
                Cow::Owned(format!("Yes\n{formatted_array}"))
            }
            None => Cow::Borrowed("No"),
        })
        .join("\n");
    println!("{output}");
}

fn solve() -> Option<Array2<usize>> {
    input! {
        (n, m): (usize, usize),
        xx: [Usize1; n],
        yy: [Usize1; m],
    }

    let mut rem_cells: BTreeMap<(usize, usize, usize), (usize, usize)> =
        iproduct!(enumerate(&xx), enumerate(&yy))
            .map(|((row, &x), (col, &y))| ((x.min(y), x, y), (row, col)))
            .collect();

    let mut aaa = Array::from_elem((n, m), n * m);
    for a in (0..n * m).rev() {
        let (&key, &coord) = rem_cells.range((a, 0, 0)..).next()?;
        rem_cells.remove(&key);
        aaa[coord] = a;
    }

    for (row, &x) in enumerate(&xx) {
        if *aaa.slice(s![row, ..]).iter().max().unwrap() != x {
            return None;
        }
    }

    for (col, &y) in enumerate(&yy) {
        if *aaa.slice(s![.., col]).iter().max().unwrap() != y {
            return None;
        }
    }

    Some(aaa)
}
