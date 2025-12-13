use itertools::Itertools;
use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut grid = Array2::from_elem([n; 2], 0);
    let mut coord = [0, (n - 1) / 2];
    for i in 0..n.pow(2) {
        grid[coord] = i + 1;

        let [r, c] = coord;
        let cand_coord_1 = [(r + n - 1) % n, (c + 1) % n];
        let cand_coord_2 = [(r + 1) % n, c];
        coord = if grid[cand_coord_1] == 0 {
            cand_coord_1
        } else {
            cand_coord_2
        };
    }

    println!(
        "{}",
        grid.axis_iter(Axis(0))
            .map(|line| line.iter().join(" "))
            .join("\n")
    );
}
