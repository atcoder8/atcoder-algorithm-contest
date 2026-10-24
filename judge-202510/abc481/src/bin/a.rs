use proconio::input;

fn main() {
    input! {
        (a, b): (u8, u8),
    }

    println!("{}", (a + b).to_string().len());
}
