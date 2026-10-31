use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xx: [i64; n],
    }

    let solve = |k: usize| {
        if k == 0 || k == n - 1 {
            return (xx[n - 1] - xx[0]) / 2;
        }

        let cand1 = (xx[k + 1] - xx[0]).max(xx[n - 1] - xx[k]) / 2;
        let cand2 = (xx[k] - xx[0]).max(xx[n - 1] - xx[k - 1]) / 2;
        cand1.min(cand2)
    };

    let output = (0..n).map(|k| solve(k)).join("\n");
    println!("{output}");
}
