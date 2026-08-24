use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, usize, u64),
        aa: [u64; n],
    }

    let mut sum = 0;
    let mut eaten = vec![false; n];
    for day in 0..n {
        if day >= m && eaten[day - m] {
            sum -= aa[day - m];
        }

        let eat = sum + aa[day] <= k;

        if eat {
            sum += aa[day];
            eaten[day] = true;
        }
    }

    let output = eaten
        .iter()
        .map(|&eat| if eat { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}
