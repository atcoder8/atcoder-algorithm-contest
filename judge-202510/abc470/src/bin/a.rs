use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let output = (1..=n)
        .map(|i| {
            if i % 3 == 0 {
                "Fizz".to_string()
            } else {
                i.to_string()
            }
        })
        .join("\n");
    println!("{output}");
}
