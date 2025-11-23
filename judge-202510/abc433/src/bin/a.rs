use proconio::input;

fn main() {
    input! {
        (x, y, z): (u32, u32, u32),
    }

    let ans = x >= y * z && (x - y * z) % (z - 1) == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
