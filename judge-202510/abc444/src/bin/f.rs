// unfinished

use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> u32 {
    input! {
        (n, m): (usize, usize),
        mut aa: [u32; n],
    }

    aa.sort_unstable();

    // 長さx以上の棒を(N+M+1)/2本以上残せるかどうか
    let is_ok = |x: u32| {
        if x == 1 {
            return true;
        }

        let border1 = aa.partition_point(|&a| a == 1);
        let border2 = aa.partition_point(|&a| a.div_ceil(2) < x);

        let mut small = VecDeque::from_iter(aa[border1..border2].iter().copied());
        let mut large = VecDeque::from_iter(aa[border2..].iter().copied());
        for _ in 0..m {
            let value = large.pop_back().or_else(|| small.pop_front()).unwrap();
            for next in [value / 2, value.div_ceil(2)] {
                if next / 2 >= x {
                    large.push_back(next);
                } else if next.div_ceil(2) >= x {
                    large.push_front(next);
                } else if next >= x {
                    small.push_back(next);
                } else if next >= 2 {
                    small.push_front(next);
                }
            }
        }

        let lt_border = small.partition_point(|&value| value < x);
        small.len() - lt_border + large.len() >= (n + m + 1) / 2
    };

    let mut ok = 1_u32;
    let mut ng = aa[n - 1] + 1;
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}
