use itertools::{Itertools, iproduct};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        ccc: [Chars; h],
    }

    let (mut top, mut bottom, mut left, mut right) = (h, 0, w, 0);
    for (row, col) in iproduct!(0..h, 0..w) {
        if ccc[row][col] == '#' {
            top = top.min(row);
            bottom = bottom.max(row);
            left = left.min(col);
            right = right.max(col);
        }
    }

    let output = (top..=bottom)
        .map(|row| (left..=right).map(|col| ccc[row][col]).collect::<String>())
        .join("\n");
    println!("{output}");
}
