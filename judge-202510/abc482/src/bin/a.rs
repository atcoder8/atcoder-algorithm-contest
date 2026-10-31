use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let parity = s.chars().filter(|&ch| ch == 'o').count() % 2 == 1;
    println!("{}", if parity { "odd" } else { "even" });
}
