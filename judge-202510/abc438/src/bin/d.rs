use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64; n],
        bb: [u64; n],
        cc: [u64; n],
    }

    let mut dp = [Some(aa[0]), None, None];
    for (&a, &b, &c) in izip!(&aa, &bb, &cc).skip(1) {
        let mut next_dp = [None::<u64>; 3];
        for pos in 0..3 {
            if let Some(prev_score) = dp[pos] {
                chmax_for_option(&mut next_dp[pos], prev_score + [a, b, c][pos]);
                if pos + 1 < 3 {
                    chmax_for_option(&mut next_dp[pos + 1], prev_score + [a, b, c][pos + 1]);
                }
            }
        }
        dp = next_dp;
    }

    println!("{}", dp[2].unwrap());
}

/// If `value` is `None` or contains a value less than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmax_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost >= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
