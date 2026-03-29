use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", if s.len() % 5 == 0 { "Yes" } else { "No" });
}
