// undefined

use itertools::{Itertools, iproduct};
use proconio::input;

fn main() {
    input! {
        (n, x, y): (usize, i64, i64),
        cab: [(char, i64, i64); n],
    }

    let mut cells = iproduct!(0..x, 0..y).collect_vec();
    for &(c, a, b) in &cab {
        if c == 'X' {
            cells.iter_mut().for_each(|(x, y)| {
                if *x < a {
                    *y += b;
                } else {
                    *y -= b;
                }
            });
        } else {
            cells.iter_mut().for_each(|(x, y)| {
                if *y < a {
                    *x += b;
                } else {
                    *x -= b;
                }
            });
        }
    }

    let mut cnt = 0_usize;
    let mut visited = vec![false; cells.len()];
    for start in 0..cells.len() {
        if visited[start] {
            continue;
        }

        cnt += 1;

        let mut queue = vec![start];
        while let Some(curr) = queue.pop() {
            if visited[curr] {
                continue;
            }

            visited[curr] = true;

            let (x, y) = cells[curr];
            queue.extend(
                [(-1, 0), (0, -1), (0, 1), (1, 0)]
                    .into_iter()
                    .filter_map(|(dx, dy)| cells.iter().position(|&cell| cell == (x + dx, y + dy))),
            );
        }
    }

    println!("{cnt}");
}
