use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, u64),
        aa: [u64; n],
    }

    let counts_by_shift = (0..=10)
        .map(|shift| {
            aa.iter()
                .map(|&a| (a % m) * (10_u64.pow(shift as u32) % m) % m)
                .sorted_unstable()
                .dedup_with_count()
                .collect_vec()
        })
        .collect_vec();
    let ans = aa
        .iter()
        .map(|&a| {
            let shift = a.to_string().len();
            let counts = &counts_by_shift[shift];
            counts
                .binary_search_by_key(&((m - a % m) % m), |&(_, rem)| rem)
                .map_or(0, |pos| counts[pos].0)
        })
        .sum::<usize>();
    println!("{ans}");
}
