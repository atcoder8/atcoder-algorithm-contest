use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> u64 {
    input! {
        (x, y, k): (u128, u128, u128),
    }

    let sub_solve = |t: u128| {
        let mut cost = 0_u64;
        let mut min = t;
        let mut max = t;
        while min < y && !(min..=max).contains(&y) {
            min *= k;
            max = k * max + (k - 1);
            cost += 1;
        }

        if (min..=max).contains(&y) {
            Some(cost)
        } else {
            None
        }
    };

    let mut t = x;
    let mut min_cost = sub_solve(x);
    for i in 1..=60 {
        t /= k;
        if let Some(c) = sub_solve(t) {
            chmin_for_option(&mut min_cost, i + c);
        }
    }
    min_cost.unwrap()
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
