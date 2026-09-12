use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, s, l): (usize, Usize1, u64),
        aa: [u64; n - 1],
    }

    let mut positions = vec![0_u64; n];
    for (i, &a) in enumerate(&aa) {
        positions[i + 1] = positions[i] + a;
    }

    let init_pos = positions[s];

    // 右端を二分探索
    let cand1 = (0..=s).filter_map(|left| {
        let d = init_pos - positions[left];
        if d > l {
            return None;
        }
        if 2 * d > l {
            return Some(s - left + 1);
        }
        let rem = l - 2 * d;
        let right = s + positions[s..].partition_point(|&a| a - init_pos <= rem);
        Some(right - left)
    });

    // 左端を二分探索
    let cand2 = (s + 1..=n).filter_map(|right| {
        let d = positions[right - 1] - init_pos;
        if d > l {
            return None;
        }
        if 2 * d > l {
            return Some(right - s);
        }
        let rem = l - 2 * d;
        let left = positions[..=s].partition_point(|&a| init_pos - a > rem);
        Some(right - left)
    });

    let ans = cand1.chain(cand2).max().unwrap();
    println!("{ans}");
}
