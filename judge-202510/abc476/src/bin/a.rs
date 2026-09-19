use proconio::input;

fn main() {
    input! {
        mut s: String,
    }

    let t = if s.chars().last() == Some('e') {
        "r"
    } else {
        "er"
    };
    s.push_str(t);

    println!("{s}");
}
