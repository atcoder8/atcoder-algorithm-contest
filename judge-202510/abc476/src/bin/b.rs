use itertools::izip;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
        t: String,
    }

    let ans = izip!(s.chars(), t.chars()).all(|(ch1, ch2)| ch2 == '*' || ch1 == ch2);
    println!("{}", if ans { "Yes" } else { "No" });
}
