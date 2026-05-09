use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, Usize1),
        aaa: [[u32]; n],
        cc: [usize; n],
    }

    let mut pos = 0;
    let mut idx = 0;
    while pos + cc[idx] * aaa[idx].len() <= k {
        pos += cc[idx] * aaa[idx].len();
        idx += 1;
    }

    println!("{}", aaa[idx][(k - pos) % aaa[idx].len()]);
}
