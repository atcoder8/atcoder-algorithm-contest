use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (n, k, x): (usize, usize, u64),
        mut aa: [u64; n],
    }

    aa.sort_unstable();
    aa.truncate(k);
    aa.reverse();

    let mut cnt = 0;
    let mut sum = 0;
    for &a in &aa {
        if sum >= x {
            break;
        }

        sum += a;
        cnt += 1;
    }

    if sum >= x { Some(n - k + cnt) } else { None }
}
