use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [Usize1; n],
    }

    println!("{}", if solve(k, aa) { "Yes" } else { "No" });
}

fn solve(k: usize, aa: Vec<usize>) -> bool {
    if aa.is_sorted() {
        return true;
    }

    let n = aa.len();

    let mut sorted_aa = aa.clone();
    sorted_aa.sort_unstable();

    let left = (0..n).find(|&i| aa[i] != sorted_aa[i]).unwrap();
    let right = (0..n).rev().find(|&i| aa[i] != sorted_aa[i]).unwrap();

    right - left + 1 <= k
}
