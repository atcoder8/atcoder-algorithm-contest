use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, v): (usize, usize),
        ww: [usize; n],
    }

    let mut max_score = 0;
    for (i, j, k) in (0..n).tuple_combinations() {
        if i + j + k + 3 <= v {
            let score = ww[i] + ww[j] + ww[k];
            max_score = max_score.max(score);
        }
    }

    println!("{max_score}");
}
