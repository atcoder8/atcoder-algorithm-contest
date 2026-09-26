use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, d): (usize, u32),
        xx: [u32; n],
    }

    let is_ok = |i: usize| (0..n).all(|j| i == j || xx[i].abs_diff(xx[j]) >= d);

    let pp = (0..n).filter(|&i| is_ok(i)).collect_vec();
    println!("{}\n{}", pp.len(), pp.iter().map(|p| p + 1).join(" "));
}
