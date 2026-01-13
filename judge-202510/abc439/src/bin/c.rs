use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut counts = vec![0_usize; n + 1];
    for x in 1..=n.sqrt() {
        let rem = n - x.pow(2);
        for y in x + 1..=rem.sqrt() {
            counts[x.pow(2) + y.pow(2)] += 1;
        }
    }

    let aa = (1..=n).filter(|&a| counts[a] == 1).collect_vec();
    println!("{}\n{}", aa.len(), aa.iter().join(" "));
}
