use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!("{}", if s.first() == s.last() { "Yes" } else { "No" });
}
