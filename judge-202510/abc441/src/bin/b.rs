use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (_n, _m): (usize, usize),
        (s, t): (String, String),
        q: usize,
        ww: [String; q],
    }

    let is_ok = |language: &str, word: &str| word.chars().all(|ch| language.contains(ch));

    let solve = |word: &str| match (is_ok(&s, word), is_ok(&t, word)) {
        (true, true) => "Unknown",
        (true, false) => "Takahashi",
        (false, true) => "Aoki",
        (false, false) => panic!(),
    };

    let output = ww.iter().map(|w| solve(w)).join("\n");
    println!("{output}");
}
