use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    println!("{s:o>n$}");
}
