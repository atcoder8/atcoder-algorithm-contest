use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    println!("{}", n + 1 - k);
}
