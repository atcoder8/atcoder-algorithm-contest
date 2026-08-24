use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ll: [u32; n],
    }

    let mut acc = vec![0_u32; n + 1];
    for (i, &l) in enumerate(&ll) {
        acc[i + 1] = acc[i] + l;
    }

    let ans = acc
        .iter()
        .map(|&prefix_sum| prefix_sum.abs_diff(acc[n] - prefix_sum))
        .min()
        .unwrap();
    println!("{ans}");
}
