use proconio::input;

fn main() {
    input! {
        (a, b): (u32, u32),
    }

    println!("{}", if 3 * a > 2 * b { "Yes" } else { "No" });
}
