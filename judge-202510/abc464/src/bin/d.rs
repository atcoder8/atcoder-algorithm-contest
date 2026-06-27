use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> i64 {
    input! {
        n: usize,
        s: Chars,
        xx: [i64; n],
        yy: [i64; n - 1],
    }

    let mut rainy = -[xx[0], 0][(s[0] == 'R') as usize];
    let mut sunny = -[xx[0], 0][(s[0] == 'S') as usize];

    for i in 1..n {
        let next_rainy = rainy.max(sunny) - [xx[i], 0][(s[i] == 'R') as usize];
        let next_sunny = (rainy + yy[i - 1]).max(sunny) - [xx[i], 0][(s[i] == 'S') as usize];
        (rainy, sunny) = (next_rainy, next_sunny);
    }

    rainy.max(sunny)
}
