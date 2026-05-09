use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aaa: [[u32]; n],
        (x, y): (Usize1, Usize1),
    }

    println!("{}", aaa[x][y]);
}
