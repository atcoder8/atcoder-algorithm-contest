use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        (n, d): (usize, i64),
        aa: [i64; n],
    }

    let can_insert = |set: &BTreeSet<(i64, usize)>, value: i64| {
        if set
            .range(..(value + 1, 0))
            .next_back()
            .is_some_and(|&(left, _)| left > value - d)
        {
            return false;
        }

        if set
            .range((value, 0)..)
            .next()
            .is_some_and(|&(right, _)| right < value + d)
        {
            return false;
        }

        true
    };

    let mut num_combs = 0_usize;
    let mut set = BTreeSet::<(i64, usize)>::new();
    let mut right = 0;
    for left in 0..n {
        while right < n && can_insert(&set, aa[right]) {
            set.insert((aa[right], right));
            right += 1;
        }

        num_combs += right - left;

        set.remove(&(aa[left], left));
    }

    println!("{num_combs}");
}
