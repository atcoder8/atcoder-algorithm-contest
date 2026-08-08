use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut score = 0;
    let mut queue = VecDeque::from_iter(s.chars());

    let mut scores = vec![0; n];
    for k in 0..n {
        while let Some(ch) = queue.pop_front() {
            score += 1;

            if ch == 'x' {
                break;
            }
        }

        scores[k] = score;
    }

    let output = scores.iter().join("\n");
    println!("{output}");
}
