use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let is_ok = |x: usize| {
        let mut cost = 0;
        for (i, &a) in enumerate(&aa) {
            cost += x.saturating_sub(a).div_ceil(i + 1);

            if cost > k {
                return false;
            }
        }

        true
    };

    let mut ok = 0_usize;
    let mut ng = 3 * 10_usize.pow(18);
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
