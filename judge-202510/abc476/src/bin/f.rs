use itertools::{Itertools, iproduct};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; n],
    }

    let mut dd = vec![0; 2 * n - 1];
    let mut ee = vec![0; 2 * n - 1];
    for (i, j) in iproduct!(0..n, 0..n) {
        let c = aa[i] * bb[j] % m;
        dd[i + j] += c;
        ee[n - 1 + i - j] += c;
    }

    let ff = (0..2 * n - 1)
        .map(|x| (0..2 * n - 1).map(|k| k.abs_diff(x) * dd[k]).sum::<usize>())
        .collect_vec();
    let gg = (0..2 * n - 1)
        .map(|x| (0..2 * n - 1).map(|k| k.abs_diff(x) * ee[k]).sum::<usize>())
        .collect_vec();

    let xor = iproduct!(0..n, 0..n).fold(0_usize, |acc, (i, j)| {
        acc ^ ((ff[i + j] + gg[n - 1 + i - j]) / 2 + i * n + j)
    });
    println!("{xor}");
}
