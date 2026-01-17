use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        pv: [(usize, u64); n],
    }

    let mut left_dp = vec![vec![0_u64; m + 1]];
    for (i, &(p, v)) in enumerate(&pv) {
        let curr_dp = &left_dp[i];
        let mut next_dp = curr_dp.clone();
        for cost in p..=m {
            next_dp[cost] = next_dp[cost].max(curr_dp[cost - p] + v);
        }
        for cost in 0..m {
            next_dp[cost + 1] = next_dp[cost + 1].max(next_dp[cost]);
        }
        left_dp.push(next_dp);
    }

    let mut right_dp = vec![vec![0_u64; m + 1]];
    for (i, &(p, v)) in enumerate(pv.iter().rev()) {
        let curr_dp = &right_dp[i];
        let mut next_dp = curr_dp.clone();
        for cost in p..=m {
            next_dp[cost] = next_dp[cost].max(curr_dp[cost - p] + v);
        }
        for cost in 0..m {
            next_dp[cost + 1] = next_dp[cost + 1].max(next_dp[cost]);
        }
        right_dp.push(next_dp);
    }
    right_dp.reverse();

    let solve = |i: usize| {
        let (p, v) = pv[i];
        let unused_max_value = (0..=m)
            .map(|left_cost| left_dp[i][left_cost] + right_dp[i + 1][m - left_cost])
            .max()
            .unwrap();
        let used_max_value = (0..=m - p)
            .map(|left_cost| left_dp[i][left_cost] + v + right_dp[i + 1][m - p - left_cost])
            .max()
            .unwrap();
        used_max_value.cmp(&unused_max_value)
    };

    let output = (0..n)
        .map(|i| match solve(i) {
            std::cmp::Ordering::Greater => 'A',
            std::cmp::Ordering::Equal => 'B',
            std::cmp::Ordering::Less => 'C',
        })
        .collect::<String>();
    println!("{output}");
}
