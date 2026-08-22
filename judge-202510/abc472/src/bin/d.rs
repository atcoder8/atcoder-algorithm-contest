use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, w, k): (usize, usize, u32),
        ss: [Chars; h],
    }

    let mut horizontal = vec![false; h];
    let mut vertical = vec![false; w];
    for (i, j) in iproduct!(0..h, 0..w) {
        if ss[i][j] == '#' {
            horizontal[i] = true;
            vertical[j] = true;
        }
    }

    let mut queue = iproduct!(0..h, 0..w)
        .map(|(i, j)| (i, j, 0_u32))
        .filter(|&(i, j, _)| !horizontal[i] && !vertical[j])
        .collect::<VecDeque<_>>();
    let mut dist_array = vec![vec![k + 1; w]; h];
    while let Some((row, col, dist)) = queue.pop_front() {
        if ss[row][col] == '#' || dist > k || dist_array[row][col] <= dist {
            continue;
        }

        dist_array[row][col] = dist;
        queue.extend(DIFFS.into_iter().filter_map(|(dr, dc)| {
            let nr = row.wrapping_add(dr);
            let nc = col.wrapping_add(dc);
            if nr < h && nc < w {
                Some((nr, nc, dist + 1))
            } else {
                None
            }
        }));
    }

    let num_safe_squares = iproduct!(0..h, 0..w)
        .filter(|&(row, col)| dist_array[row][col] <= k)
        .count();
    println!("{num_safe_squares}");
}
