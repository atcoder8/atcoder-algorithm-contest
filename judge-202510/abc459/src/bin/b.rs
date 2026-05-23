use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let get_label = |ch: char| match ch {
        'a' | 'b' | 'c' => 2,
        'd' | 'e' | 'f' => 3,
        'g' | 'h' | 'i' => 4,
        'j' | 'k' | 'l' => 5,
        'm' | 'n' | 'o' => 6,
        'p' | 'q' | 'r' | 's' => 7,
        't' | 'u' | 'v' => 8,
        'w' | 'x' | 'y' | 'z' => 9,
        _ => panic!(),
    };

    let cc = ss
        .iter()
        .map(|s| get_label(s.chars().next().unwrap()))
        .join("");
    println!("{cc}");
}
