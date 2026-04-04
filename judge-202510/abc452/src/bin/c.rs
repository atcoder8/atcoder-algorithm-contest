use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        ab: [(usize, Usize1); n],
        m: usize,
        ss: [Chars; m],
    }

    let create_exist_list = |len: usize, pos: usize| {
        let mut contains = vec![false; 26];
        for s in &ss {
            if s.len() == len {
                contains[char_to_int(s[pos])] = true;
            }
        }

        contains
    };

    let list_by_lib = ab
        .iter()
        .map(|&(a, b)| create_exist_list(a, b))
        .collect_vec();

    let is_ok = |s: &[char]| s.len() == n && (0..n).all(|i| list_by_lib[i][char_to_int(s[i])]);

    let output = ss
        .iter()
        .map(|s| if is_ok(s) { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
