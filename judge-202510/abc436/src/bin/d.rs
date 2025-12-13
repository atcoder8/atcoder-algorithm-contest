use std::collections::VecDeque;

use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w): (usize, usize),
        sss: [Chars; h],
    }

    let mut warp_groups = vec![vec![]; 26];
    for (row, col) in iproduct!(0..h, 0..w) {
        let ch = sss[row][col];
        if ('a'..='z').contains(&ch) {
            warp_groups[char_to_int(ch)].push((row, col));
        }
    }

    let mut cost_array = Array2::from_elem((h, w), None::<usize>);
    let mut queue = VecDeque::from([((0, 0), 0)]);
    let mut warped = vec![false; 26];
    while let Some((coord, cost)) = queue.pop_front() {
        if cost_array[coord].is_some() {
            continue;
        }

        cost_array[coord] = Some(cost);

        let (row, col) = coord;
        for (dr, dc) in DIFFS {
            let adj_row = row.wrapping_add(dr);
            let adj_col = col.wrapping_add(dc);
            if adj_row < h && adj_col < w && sss[adj_row][adj_col] != '#' {
                queue.push_back(((adj_row, adj_col), cost + 1));
            }
        }

        let ch = sss[row][col];
        if ('a'..='z').contains(&ch) {
            let u = char_to_int(ch);
            if !warped[u] {
                warped[u] = true;
                queue.extend(warp_groups[u].iter().map(|&coord| (coord, cost + 1)));
            }
        }
    }

    match cost_array[(h - 1, w - 1)] {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
