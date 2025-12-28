use itertools::{Itertools, izip};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        s: String,
        t: String,
    }

    let s = s.chars().map(|ch| ch.to_digit(10).unwrap()).collect_vec();
    let t = t.chars().map(|ch| ch.to_digit(10).unwrap()).collect_vec();

    let calc_cost = |start: usize| {
        izip!(&s[start..], &t)
            .map(|(&ch1, &ch2)| (10 + ch1 - ch2) % 10)
            .sum::<u32>()
    };

    let min_cost = (0..=n - m).map(calc_cost).min().unwrap();
    println!("{min_cost}");
}
