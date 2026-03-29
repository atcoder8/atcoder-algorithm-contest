use itertools::Itertools;
use proconio::input;

const MAX: u64 = 10_u64.pow(9);

fn main() {
    input! {
        n: usize,
    }

    let seeds = (0_u32..30).map(|exp| 2_u64.pow(exp)).collect_vec();

    let mut pool = vec![];
    let mut stack = vec![0];
    while let Some(value) = stack.pop() {
        pool.push(value);
        stack.extend(
            seeds
                .iter()
                .map(|seed| value * 10_u64.pow(seed.ilog10() + 1) + seed)
                .filter(|&next_value| next_value <= MAX),
        );
    }

    pool.sort_unstable();
    pool.dedup();

    let ans = *pool.iter().nth(n).unwrap();
    println!("{ans}");
}
