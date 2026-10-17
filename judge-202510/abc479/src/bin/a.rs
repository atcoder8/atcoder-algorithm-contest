use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let sum = aa.iter().step_by(2).sum::<u32>();
    println!("{sum}");
}
