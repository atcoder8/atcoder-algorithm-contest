use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, mut x): (usize, u8),
        aa: [u8; n],
    }

    let output = aa
        .iter()
        .map(|&a| {
            if a < x {
                x = a;
                1
            } else {
                0
            }
        })
        .join("\n");
    println!("{output}");
}
