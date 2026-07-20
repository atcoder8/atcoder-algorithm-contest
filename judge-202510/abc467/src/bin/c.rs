use proconio::input;

fn main() {
    input! {
        (n, _m): (usize, u32),
        aa: [u32; n],
        bb: [u32; n - 1],
    }

    let mut dp = [0, 1];
    for i in 0..n - 1 {
        let parity = (aa[i] ^ aa[i + 1] ^ bb[i]) as usize;
        dp = [dp[parity], dp[1 - parity] + 1];
    }

    println!("{}", dp[0].min(dp[1]));
}
