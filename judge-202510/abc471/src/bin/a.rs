use proconio::input;

fn main() {
    input! {
        (a, b): (i32, i32),
    }

    let ans = [a + b, a - b, a * b].iter().any(|&value| value == 9) || a == 9 * b;
    println!("{}", if ans { "Nine" } else { "Nein" });
}
