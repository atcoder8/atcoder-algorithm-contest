use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut cost = 0;
    let mut pos_t = 0;
    for &ch in &s {
        if pos_t < t.len() && ch == t[pos_t] {
            pos_t += 1;
            continue;
        }

        if ch == 'A' {
            cost += 1;
            continue;
        }

        while pos_t < t.len() && t[pos_t] == 'A' {
            cost += 1;
            pos_t += 1;
        }

        if pos_t < t.len() && t[pos_t] == ch {
            pos_t += 1;
        } else {
            return None;
        }
    }

    if t[pos_t..].iter().any(|&ch| ch != 'A') {
        return None;
    }

    cost += t.len() - pos_t;

    Some(cost)
}
