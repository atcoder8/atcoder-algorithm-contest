use proconio::input;

fn main() {
    input! {
        (b, r): (usize, usize),
    }

    println!("{}", (30 - b) * (30 - r));
}
