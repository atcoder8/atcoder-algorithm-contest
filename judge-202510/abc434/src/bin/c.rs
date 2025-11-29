use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t)
        .map(|_| if solve() { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}

fn solve() -> bool {
    input! {
        (n, h): (usize, u64),
        tlu: [(u64, u64, u64); n],
    }

    let mut prev_t = 0;
    let mut min = h;
    let mut max = h;
    for &(t, l, u) in &tlu {
        let diff_t = t - prev_t;
        min = min.saturating_sub(diff_t).max(l);
        max = (max + diff_t).min(u);
        if min > u || max < l {
            return false;
        }
        prev_t = t;
    }

    true
}
