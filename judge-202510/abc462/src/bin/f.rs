use itertools::{Itertools, izip};
use proconio::input;

fn main() {
    input! {
        t: usize,
        sk: [(String, usize); t],
    }

    let output = sk
        .iter()
        .map(|&(ref s, k)| solve(s, k).map_or("-1".to_string(), |cost| cost.to_string()))
        .join("\n");
    println!("{output}");
}

fn solve(s: &str, k: usize) -> Option<usize> {
    let mut dp = vec![vec![None::<usize>; k + 1]; s.len() + 1];
    dp[0][0] = Some(0);
    for i in 0..s.len() {
        for inc in 0..=k {
            let Some(cost) = dp[i][inc] else {
                continue;
            };

            let step = if s[i..].starts_with("ABC") { 3 } else { 1 };
            chmin_for_option(&mut dp[i + step][inc], cost);

            if i + 2 < s.len() && inc < k {
                let add_cost = izip!(s[i..i + 3].chars(), "ABC".chars())
                    .filter(|(ch1, ch2)| ch1 != ch2)
                    .count();
                let removed = (0..=2).any(|offset| s[i + offset..].starts_with("ABC"));
                chmin_for_option(&mut dp[i + 3][inc + !removed as usize], cost + add_cost);
            }
        }
    }

    dp[s.len()][k]
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
