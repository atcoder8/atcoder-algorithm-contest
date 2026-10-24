use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, u32),
        aa: [u32; n],
    }

    let mut cnt = (aa[n - 1] == k) as usize;
    for (&a1, &a2) in aa.iter().tuple_windows() {
        if a1 < a2 {
            cnt += (a1..a2).contains(&k) as usize;
        } else {
            cnt += (a2 + 1..=a1).contains(&k) as usize;
        }
    }

    println!("{cnt}");
}
