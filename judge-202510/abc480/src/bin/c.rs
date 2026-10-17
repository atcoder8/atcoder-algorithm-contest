use proconio::input;

fn main() {
    input! {
        (n, k, t): (usize, u64, u64),
        mut aa: [u64; n],
    }

    aa.retain(|&a| a < t);
    aa.sort_unstable();

    let mut cnt = 0;
    let mut curr = 0;
    for &a in &aa {
        cnt += a.saturating_sub(curr).div_ceil(k);
        curr = a + k;
    }
    cnt += t.saturating_sub(curr).div_ceil(k);

    println!("{cnt}");
}
