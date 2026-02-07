use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [u32; n],
    }

    aa.sort_unstable();

    let mut candidates = vec![];

    // L=max(A)
    let num_broken = aa.partition_point(|&a| a < aa[n - 1]);
    if num_broken % 2 == 0
        && (0..num_broken / 2).all(|i| aa[i] + aa[num_broken - 1 - i] == aa[n - 1])
    {
        candidates.push(aa[n - 1]);
    }

    // L=min(A)+max(A)
    if n % 2 == 0 && (1..n / 2).all(|i| aa[i] + aa[n - 1 - i] == aa[0] + aa[n - 1]) {
        candidates.push(aa[0] + aa[n - 1]);
    }

    println!("{}", candidates.iter().join(" "));
}
