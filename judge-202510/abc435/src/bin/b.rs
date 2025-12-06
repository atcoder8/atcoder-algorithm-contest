use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let is_ok = |seq: &[u32]| {
        let sum = seq.iter().sum::<u32>();
        seq.iter().all(|&v| sum % v != 0)
    };

    let ans = (0..=n)
        .tuple_combinations()
        .filter(|&(l, r)| is_ok(&aa[l..r]))
        .count();
    println!("{ans}");
}
