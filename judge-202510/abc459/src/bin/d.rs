use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        ss: [String; t],
    }

    let output = ss
        .iter()
        .map(|s| match solve(s) {
            Some(rearranged) => format!("Yes\n{rearranged}"),
            None => "No".to_string(),
        })
        .join("\n");
    println!("{output}");
}

fn solve(s: &str) -> Option<String> {
    let sorted = s
        .chars()
        .sorted_unstable()
        .dedup_with_count()
        .sorted_unstable_by_key(|v| Reverse(v.0))
        .flat_map(|(cnt, ch)| vec![ch; cnt])
        .collect::<String>();

    let mut rearranged = vec!['\0'; s.len()];
    let mut chars = sorted.chars();
    for i in (0..s.len()).step_by(2) {
        rearranged[i] = chars.next().unwrap();
    }
    for i in (1..s.len()).step_by(2) {
        rearranged[i] = chars.next().unwrap();
    }

    if rearranged
        .iter()
        .tuple_windows()
        .all(|(ch1, ch2)| ch1 != ch2)
    {
        Some(rearranged.iter().collect())
    } else {
        None
    }
}
