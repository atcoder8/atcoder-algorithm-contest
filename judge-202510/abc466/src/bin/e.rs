use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ab: [(u64, u64); n],
    }

    let max_sections = 2 * k + 1;

    let mut dp = vec![0_u64; max_sections + 1];
    for &(a, b) in &ab {
        for section in (0..max_sections).rev() {
            let mut updated_scores = [dp[section], dp[section + 1]];
            for next_facing in [0, 1] {
                let add_score = [a, b][next_facing];
                let offset = (section % 2).abs_diff(next_facing);
                updated_scores[offset] = updated_scores[offset].max(dp[section] + add_score);
            }
            dp[section..section + 2].copy_from_slice(&updated_scores);
        }
    }

    let max_score = dp[..max_sections].iter().max().unwrap();
    println!("{max_score}");
}
