use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        abcd: [(u64, u64, i64, i64); t],
    }

    let output = abcd
        .iter()
        .map(|&(a, b, x, y)| solve(a, b, x, y))
        .join("\n");
    println!("{output}");
}

fn solve(mut a: u64, mut b: u64, x: i64, y: i64) -> u64 {
    let mut x = x.abs() as u64;
    let mut y = y.abs() as u64;

    if x < y {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut y);
    }

    let d = x - y;

    let first_half_cost = a.min(b) * 2 * y;
    let second_half_cost_candidates = [
        a * d.div_ceil(2) + b * (d / 2),
        a * (2 * d - d % 2),
        b * (2 * d + d % 2),
    ];

    first_half_cost + second_half_cost_candidates.into_iter().min().unwrap()
}
