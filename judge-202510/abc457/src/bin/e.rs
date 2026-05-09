// unfinished

use itertools::{Itertools, enumerate, iproduct};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(usize, usize); m],
        q: usize,
        st: [(usize, usize); q],
    }

    let mut rs = vec![vec![]; n + 1];
    let mut ls = vec![vec![]; n + 1];
    for (i, &(l, r)) in enumerate(&lr) {
        rs[l].push((r, i));
        ls[r].push((l, i));
    }
    rs.iter_mut().for_each(|rr| rr.sort_unstable());
    ls.iter_mut().for_each(|ll| ll.sort_unstable());

    let solve = |s: usize, t: usize| {
        let rr = &rs[s];
        let max_r_idx = rr.partition_point(|ri| ri.0 <= t);
        let candidate_rs = &rr[max_r_idx.saturating_sub(2)..max_r_idx];

        let ll = &ls[t];
        let min_l_idx = ll.partition_point(|li| li.0 < s);
        let candidate_ls = &ll[min_l_idx..(min_l_idx + 2).min(ll.len())];

        iproduct!(candidate_rs, candidate_ls).any(|(&(r, i), &(l, j))| r + 1 >= l && i != j)
    };

    let output = st
        .iter()
        .map(|&(s, t)| if solve(s, t) { "Yes" } else { "No" })
        .join("\n");
    println!("{output}");
}
