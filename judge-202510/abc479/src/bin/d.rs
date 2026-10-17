// s < t のとき
// min(R_s, L_s + min(R_1, R_2, ... , R_{s-1}))
// s > t のとき
// min(L_s, R_s + min(L_{s+1}, L_{s+2}, ... , L_N))

use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: u64 = 10_u64.pow(10);

fn main() {
    input! {
        (n, q): (usize, usize),
        mut ll: [u64; n - 1],
        mut rr: [u64; n - 1],
        st: [(Usize1, Usize1); q],
    }

    let mut acc_l = vec![INF; n];
    for (i, &l) in ll.iter().enumerate().rev() {
        acc_l[i] = acc_l[i + 1].min(l);
    }

    let mut acc_r = vec![INF; n];
    for (i, &r) in rr.iter().enumerate() {
        acc_r[i + 1] = acc_r[i].min(r);
    }

    ll.insert(0, INF);
    rr.push(INF);

    let solve = |s: usize, t: usize| {
        if s < t {
            rr[s].min(ll[s] + acc_r[s])
        } else {
            ll[s].min(rr[s] + acc_l[s])
        }
    };

    let output = st.iter().map(|&(s, t)| solve(s, t)).join("\n");
    println!("{output}");
}
