use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        ccc: [Chars; h],
    }

    let is_painted_row = |row: usize| (0..w).any(|col| ccc[row][col] == '#');
    let is_painted_column = |col: usize| (0..h).any(|row| ccc[row][col] == '#');

    let top = (0..h).find(|&row| is_painted_row(row)).unwrap();
    let bottom = (0..h).rev().find(|&row| is_painted_row(row)).unwrap();
    let left = (0..w).find(|&col| is_painted_column(col)).unwrap();
    let right = (0..w).rev().find(|&col| is_painted_column(col)).unwrap();

    let output = (top..=bottom)
        .map(|row| (left..=right).map(|col| ccc[row][col]).collect::<String>())
        .join("\n");
    println!("{output}");
}
