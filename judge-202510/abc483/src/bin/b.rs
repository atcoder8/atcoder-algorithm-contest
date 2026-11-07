use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, q): (usize, usize),
        kk: [usize; q],
    }

    let mut cards = (1..=n as i32).collect_vec();
    for &k in &kk {
        cards[..k].reverse();
        cards[..k].iter_mut().for_each(|card| *card = -*card);
    }

    let output = cards.iter().join("\n");
    println!("{output}");
}
