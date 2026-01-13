use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    println!("{}", 2_u32.pow(n) - 2 * n);
}
