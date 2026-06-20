use proconio::input;

const MAX: u32 = 10_u32.pow(9);

fn main() {
    input! {
        (n, k): (usize, usize),
        mut lr: [(u32, u32); n],
    }

    lr.sort_unstable_by_key(|v| v.1);

    let is_ok = |x: u32| {
        let mut curr = lr[0].1;
        let mut idx = 1;
        for _ in 1..k {
            while idx < n && lr[idx].0 < curr + x {
                idx += 1;
            }

            if idx == n {
                return false;
            }

            curr = lr[idx].1;
        }

        true
    };

    let solve = || {
        if !is_ok(1) {
            return None;
        }

        let mut ok = 0_u32;
        let mut ng = MAX + 1;
        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        Some(ok)
    };

    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}
