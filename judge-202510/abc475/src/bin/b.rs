use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64; n],
    }

    let mut counts = [0; 3];
    for &a in &aa {
        let rem = (1000 - a % 1000) % 1000;
        for exp in 0..3 {
            counts[exp] += rem % 10_u64.pow(exp as u32 + 1) / 10_u64.pow(exp as u32);
        }
    }

    let output = counts.iter().join(" ");
    println!("{output}");
}
