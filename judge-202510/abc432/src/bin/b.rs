use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut x: String,
    }

    let mut chars = x.chars().sorted_unstable().collect_vec();
    let pos = chars.iter().position(|&ch| ch != '0').unwrap();
    let head = chars.remove(pos);
    println!("{head}{}", chars.iter().collect::<String>());
}
