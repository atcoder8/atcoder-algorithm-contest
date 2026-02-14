use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let max_len = ss.iter().map(|s| s.len()).max().unwrap();
    let output = ss.iter().map(|s| format!("{s:.^max_len$}")).join("\n");
    println!("{output}");
}
