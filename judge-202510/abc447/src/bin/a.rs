use proconio::input;

fn main() {
    input! {
        (n, m): (u8, u8),
    }

    println!("{}", if n >= 2 * m - 1 { "Yes" } else { "No" });
}
