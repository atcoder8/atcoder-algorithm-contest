use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let count_equiv_combinations = |ignore_positions: &[usize]| {
        let masked = ss.iter().map(|s| {
            let mut t = s.clone();
            ignore_positions.iter().for_each(|&pos| t[pos] = '*');
            t
        });
        masked
            .counts()
            .values()
            .map(|cnt| cnt * (cnt - 1) / 2)
            .sum::<usize>()
    };

    let count_by_distance = |dist: usize| {
        (0..m)
            .combinations(dist)
            .map(|positions| count_equiv_combinations(&positions))
            .sum::<usize>()
    };

    let [cnt0, cnt1, cnt2] = [0, 1, 2].map(count_by_distance);

    let num_combs = (m - 1) * (m - 2) / 2 * cnt0 + cnt2 - (m - 2) * cnt1;
    println!("{num_combs}");
}
