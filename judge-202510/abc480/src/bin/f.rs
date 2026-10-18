use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let s = aa.iter().sum::<usize>();
    let t = s / m as usize;

    let mut dp = vec![m; 1 << n];
    dp[0] = 0;
    for bits in 0..1 << n {
        let rem = (0..n).fold(0, |acc, i| (acc + aa[i] * (bits >> i & 1)) % t);

        for i in 0..n {
            if bits >> i & 1 == 1 {
                continue;
            }

            let next_bits = bits | (1 << i);
            dp[next_bits] = dp[next_bits].min(dp[bits] + (rem + aa[i] - 1) / t);
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
