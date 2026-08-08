use itertools::{Itertools, chain};
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = chain!(['x'], s.chars(), ['x'])
        .tuple_windows()
        .filter(|&(c1, c2, c3)| [c1, c2, c3] == ['x'; 3])
        .count();
    println!("{ans}");
}
