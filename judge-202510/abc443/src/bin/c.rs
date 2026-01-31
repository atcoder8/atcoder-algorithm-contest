use itertools::chain;
use proconio::input;

fn main() {
    input! {
        (n, t): (usize, u64),
        aa: [u64; n],
    }

    let mut sum_time = 0;
    let mut time = 0;
    for &a in chain!(&aa, [&t]) {
        if time < a {
            sum_time += a - time;
            time = a + 100;
        }
    }
    println!("{sum_time}");
}
