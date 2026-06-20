use num::Integer;
use proconio::input;

fn main() {
    input! {
        (x, y): (u32, u32),
    }

    let gcd = x.gcd(&y);
    println!(
        "{}",
        if (x / gcd, y / gcd) == (16, 9) {
            "Yes"
        } else {
            "No"
        }
    );
}
