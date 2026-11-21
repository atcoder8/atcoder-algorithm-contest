use itertools::Itertools;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        n: usize,
        s: String,
    }

    let seq = s.chars().map(char_to_int).collect_vec();
    let mut num_per_char = [0_usize; 3];
    for &value in &seq {
        num_per_char[value] += 1;
    }

    let mut bitset = 0_u8;
    let leftmost = (0..n)
        .find(|&i| {
            bitset |= 1 << seq[i];
            bitset == 7
        })
        .map(|v| v + 1);
    let Some(leftmost) = leftmost else { return 0 };

    let mut bitset = 0_u8;
    let rightmost = (0..n)
        .rev()
        .find(|&i| {
            bitset |= 1 << seq[i];
            bitset == 7
        })
        .unwrap();

    let mut num_combs = 0_usize;
    let mut right = leftmost;
    let mut counts = [0_usize; 3];
    for left in leftmost..n {
        while right <= rightmost && counts.iter().any(|&cnt| cnt == 0) {
            counts[seq[right]] += 1;
            right += 1;
        }
        if right > rightmost {
            break;
        }
        num_combs += rightmost - right + 1;

        counts[seq[left]] -= 1;
    }

    num_combs
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'A') as usize
}
