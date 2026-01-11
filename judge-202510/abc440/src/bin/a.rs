use proconio::input;

fn main() {
    input! {
        (x, y): (u32, u32),
    }

    println!("{}", x << y);
}
