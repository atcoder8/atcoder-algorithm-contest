use proconio::input;

fn main() {
    input! {
        s: String,
        n: usize,
    }

    println!("{}", &s[n..s.len() - n]);
}
