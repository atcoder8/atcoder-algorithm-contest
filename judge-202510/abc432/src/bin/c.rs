use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, x, y): (usize, usize, usize),
        aa: [usize; n],
    }

    let d = y - x;
    let min_a = *aa.iter().min().unwrap();

    let mut sum = 0_usize;
    for &a in &aa {
        let diff_num_candies = (a - min_a) * y;

        if a < diff_num_candies / d || diff_num_candies % d != 0 {
            return None;
        }

        sum += a - diff_num_candies / d;
    }

    Some(sum)
}
