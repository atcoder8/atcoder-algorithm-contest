use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        sss: [Chars; 2],
    }

    let find_coord = |ch: char| {
        iproduct!(0..2, 0..2)
            .find(|&(row, col)| sss[row][col] == ch)
            .unwrap()
    };

    let (ra, ca) = find_coord('A');
    let (rd, cd) = find_coord('D');

    let ans = ra == rd || ca == cd;
    println!("{}", if ans { "Yes" } else { "No" });
}
