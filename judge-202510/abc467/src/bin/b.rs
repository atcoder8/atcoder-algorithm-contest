use proconio::input;

fn main() {
    input! {
        n: usize,
        abs: [(u32, u32, String); n],
    }

    let ans = abs
        .into_iter()
        .map(|(a, b, s)| if s == "keep" { b - a } else { 0 })
        .sum::<u32>();
    println!("{ans}");
}
