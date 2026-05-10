use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some(t) => println!("Yes\n{t}"),
        None => println!("No"),
    }
}

fn solve() -> Option<String> {
    input! {
        (n, m): (usize, usize),
        mut ss: [Chars; n],
    }

    ss.sort_unstable();

    let mut stack = vec![(String::new(), 0, (0..n).collect_vec())];
    while let Some((prefix, pos, rem)) = stack.pop() {
        if rem.is_empty() {
            let t = format!("{prefix}{}", "0".repeat(m - prefix.len()));
            return Some(t);
        }

        if pos == m || 2_usize.pow(pos as u32) > n {
            continue;
        }

        let (rem0, rem1) = rem.iter().partition(|&&i| ss[i][pos] != '0');
        stack.extend(
            [('0', rem0), ('1', rem1)]
                .into_iter()
                .map(|(ch, next_rem)| (format!("{prefix}{ch}"), pos + 1, next_rem)),
        );
    }

    None
}
