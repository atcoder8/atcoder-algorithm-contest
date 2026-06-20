use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, x): (usize, char),
        ss: [Chars; n],
    }

    let i = char_to_int(x);
    let ans = ss.iter().any(|s| s[i] == 'o');
    println!("{}", if ans { "Yes" } else { "No" });
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'A') as usize
}
