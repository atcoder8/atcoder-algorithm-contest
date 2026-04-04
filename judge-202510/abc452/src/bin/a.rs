use proconio::input;

const GOTHEC: [(u8, u8); 5] = [(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)];

fn main() {
    input! {
        md: (u8, u8),
    }

    println!("{}", if GOTHEC.contains(&md) { "Yes" } else { "No" });
}
