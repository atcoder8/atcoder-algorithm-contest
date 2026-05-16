use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
    }

    let count = |row: usize, col: usize| {
        [row > 0, row < h - 1, col > 0, col < w - 1]
            .into_iter()
            .filter(|&v| v)
            .count()
    };

    let output = (0..h)
        .map(|row| (0..w).map(|col| count(row, col)).join(" "))
        .join("\n");
    println!("{output}");
}
