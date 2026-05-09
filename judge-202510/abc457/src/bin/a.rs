use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [u8; n],
        x: Usize1,
    }

    println!("{}", aa[x]);
}
