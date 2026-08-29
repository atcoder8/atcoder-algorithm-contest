use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let sum = aa[n / 2..].iter().sum::<u32>();
    println!("{sum}");
}
