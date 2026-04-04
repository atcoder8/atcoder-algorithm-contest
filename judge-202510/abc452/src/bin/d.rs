use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // 各小文字cについてs[i]==cを満たすiの集合を求める
    let mut indexes = [const { vec![] }; 26];
    for (i, &ch) in enumerate(&s) {
        indexes[char_to_int(ch)].push(i);
    }

    let get_max_length = |left: usize| {
        let mut right = left;
        for &ch in &t {
            let indexes = &indexes[char_to_int(ch)];
            let pos = indexes.partition_point(|&idx| idx < right);
            if pos == indexes.len() {
                right = s.len() + 1;
                break;
            }
            right = indexes[pos] + 1;
        }

        right - left - 1
    };

    let num_combs = (0..s.len()).map(get_max_length).sum::<usize>();
    println!("{num_combs}");
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}
