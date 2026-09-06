use proconio::input;

fn main() {
    input! {
        x: u8,
    }

    println!("{}", x % 3 + 1);
}
