use proconio::input;

fn main() {
    input! {
        (p, q): (u32, u32),
        (x, y): (u32, u32),
    }

    let ans = (p..p + 100).contains(&x) && (q..q + 100).contains(&y);
    println!("{}", if ans { "Yes" } else { "No" });
}
