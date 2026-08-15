use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut coords = aa.into_iter().collect::<BTreeSet<i64>>();
    let mut x = 0;

    let mut total = 0;
    for _ in 0..n {
        let prev = coords.range(..=x).next_back();
        let next = coords.range(x..).next();
        let nx = match (prev, next) {
            (None, None) => break,
            (None, Some(&next)) => next,
            (Some(&prev), None) => prev,
            (Some(&prev), Some(&next)) => {
                let dist1 = prev.abs_diff(x);
                let dist2 = next.abs_diff(x);
                match dist1.cmp(&dist2) {
                    std::cmp::Ordering::Less => prev,
                    std::cmp::Ordering::Equal => prev,
                    std::cmp::Ordering::Greater => next,
                }
            }
        };

        total += x.abs_diff(nx);

        coords.remove(&nx);
        x = nx;
    }

    println!("{total}");
}
