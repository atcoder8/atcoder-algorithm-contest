use proconio::input;

fn main() {
    input! {
        n: usize,
        xx: [i32; n],
    }

    let ans = xx.iter().all(|&x| x < 0);
    println!("{}", if ans { "Yes" } else { "No" });
}
