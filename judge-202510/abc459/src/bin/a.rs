use proconio::{input, marker::Usize1};

fn main() {
    input! {
        x: Usize1,
    }

    let s = "HelloWorld";
    println!("{}{}", &s[..x], &s[x + 1..]);
}
