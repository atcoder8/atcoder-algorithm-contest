use proconio::input;

fn main() {
    input! {
        x: u8,
    }

    println!("{}", if 3 <= x && x <= 18 { "Yes" } else { "No" });
}
