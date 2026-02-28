use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut counts = [0_usize; 26];
    for ch in s.chars() {
        counts[char_to_int(ch)] += 1;
    }

    let max_cnt = *counts.iter().max().unwrap();

    let ans = s
        .chars()
        .filter(|&ch| counts[char_to_int(ch)] != max_cnt)
        .collect::<String>();
    println!("{ans}");
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
