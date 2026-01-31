use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> String {
    input! {
        (n, c): (usize, Usize1),
        mut ss: [Chars; n],
    }

    ss.reverse();

    let mut breakable = vec![false; n];
    let limit_by_col = (0..n)
        .map(|col| (0..n).find(|&row| ss[row][col] == '#').unwrap_or(n))
        .collect_vec();
    let mut dp = vec![false; n];
    dp[c] = true;
    let mut next_dp = vec![false; n];
    for row in 0..n - 1 {
        for col in 0..n {
            if !dp[col] {
                continue;
            }

            for adjacent_col in [col.wrapping_add(!0), col, col + 1] {
                if adjacent_col >= n {
                    continue;
                }

                if row < limit_by_col[adjacent_col] {
                    breakable[adjacent_col] = true;
                }

                if ss[row + 1][adjacent_col] == '.' || breakable[adjacent_col] {
                    next_dp[adjacent_col] = true;
                }
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
        next_dp.fill(false);
    }

    dp.iter()
        .map(|&success| if success { '1' } else { '0' })
        .collect()
}
