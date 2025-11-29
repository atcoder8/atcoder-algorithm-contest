use proconio::input;

fn main() {
    input! {
        (w, b): (usize, usize),
    }

    println!("{}", 1000 * w / b + 1);
}
