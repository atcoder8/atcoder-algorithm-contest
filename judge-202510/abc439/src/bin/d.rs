use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [u64; n],
    }

    let cnt1 = count(&aa);
    aa.reverse();
    let cnt2 = count(&aa);
    println!("{}", cnt1 + cnt2);
}

fn count(aa: &[u64]) -> usize {
    let mut cnt = 0;
    let mut pool = BTreeMap::<u64, usize>::new();
    for &a in aa {
        if a % 5 == 0 {
            let divided = a / 5;
            cnt += pool.get(&(divided * 7)).copied().unwrap_or(0)
                * pool.get(&(divided * 3)).copied().unwrap_or(0);
        }

        *pool.entry(a).or_insert(0) += 1;
    }

    cnt
}
