use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans: String = s.chars().filter(|&ch| ch.is_numeric()).collect();
    println!("{ans}");
}
