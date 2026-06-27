use itertools::{Itertools, enumerate, iproduct};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, q): (usize, usize, usize),
        rcx: [(Usize1, Usize1, char); q],
    }
    let mut grid = vec![vec![0_usize; w]; h];
    for (i, &(r, c, _)) in enumerate(&rcx) {
        grid[r][c] = i + 1;
    }

    for (row, col) in iproduct!((0..h).rev(), (0..w).rev()) {
        if row + 1 < h {
            grid[row][col] = grid[row][col].max(grid[row + 1][col]);
        }
        if col + 1 < w {
            grid[row][col] = grid[row][col].max(grid[row][col + 1]);
        }
    }

    let mut colors = rcx.iter().map(|v| v.2).collect_vec();
    colors.insert(0, 'A');

    let output = grid
        .iter()
        .map(|row| row.iter().map(|&idx| colors[idx]).collect::<String>())
        .join("\n");
    println!("{output}");
}
