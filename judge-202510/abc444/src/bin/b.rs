use proconio::input;

fn main() {
    input! {
        (n, k): (u32, u32),
    }

    let ans = (1..=n).filter(|&i| calc_digit_sum(i) == k).count();
    println!("{ans}");
}

fn calc_digit_sum(n: u32) -> u32 {
    n.to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .sum()
}
