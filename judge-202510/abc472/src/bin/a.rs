use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String,
    }

    let re = Regex::new(r"[^A]").unwrap();
    let replaced = re.replace_all(&s, ".");
    println!("{replaced}");
}
