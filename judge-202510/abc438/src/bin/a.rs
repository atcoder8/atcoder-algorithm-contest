use proconio::input;

fn main() {
    input! {
        (d, f): (u32, u32),
    }

    println!("{}", 7 - (d - f) % 7);
}
