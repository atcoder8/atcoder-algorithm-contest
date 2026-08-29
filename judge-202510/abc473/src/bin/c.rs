use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [Usize1; n],
    }

    let mut counts = vec![0_usize; k];
    for &a in &aa {
        counts[a] += 1;
    }

    let max_cnt = *counts.iter().max().unwrap();

    let ans = counts.iter().filter(|&&cnt| cnt + 1 >= max_cnt).count();
    println!("{ans}");
}
