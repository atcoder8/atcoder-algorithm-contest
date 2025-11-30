use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> usize {
    input! {
        aa: [usize; 9],
    }

    let sum_a = aa.iter().sum::<usize>();

    if aa[4] > 0 {
        (aa[4] - 1).saturating_sub(sum_a - aa[4])
    } else {
        ((1..=4).filter(|&i| aa[i - 1] > 0 && aa[9 - i] > 0).count() == 1) as usize
    }
}
