use proconio::input;

fn main() {
    input! {
        (h, w): (u32, u32),
    }

    let ans = 10000 * w >= 25 * h.pow(2);
    println!("{}", if ans { "Yes" } else { "No" });
}
