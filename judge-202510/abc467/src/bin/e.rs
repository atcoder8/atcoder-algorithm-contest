// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; n - 1],
    }

    let even_counter = (0..n - 1)
        .step_by(2)
        .map(|i| (bb[i] + 2 * m - (aa[i] + aa[i + 1])) % m)
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();
    let odd_counter = (1..n - 1)
        .step_by(2)
        .map(|i| (bb[i] + 2 * m - (aa[i] + aa[i + 1])) % m)
        .sorted_unstable()
        .dedup_with_count()
        .collect_vec();

    let candidates = (0..n - 1)
        .map(|i| (bb[i] + 2 * m - (aa[i] + aa[i + 1])) % m)
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let mut cost = (0..n - 1)
        .map(|i| (bb[i] + 2 * m - (aa[i] + aa[i + 1])) % m)
        .sum::<usize>();
    let mut min_cost = cost;

    let mut prev = 0;
    let mut plus_cnt = odd_counter.len();
    let mut even_idx = 0;
    let mut odd_idx = 0;
    for &cand in &candidates {
        cost -= (n - 1 - plus_cnt) * (cand - prev);
        cost += plus_cnt * (cand - prev);
        min_cost = min_cost.min(cand + cost);

        while even_idx < even_counter.len() && even_counter[even_idx].1 < cand {
            plus_cnt += even_counter[even_idx].0;
            even_idx += 1;
        }
        while odd_idx < odd_counter.len() && odd_counter[odd_idx].1 < cand {
            plus_cnt -= odd_counter[odd_idx].0;
            odd_idx += 1;
        }
        prev = cand;
    }

    println!("{min_cost}");
}
