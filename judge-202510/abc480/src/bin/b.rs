use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [String; n],
        tt: [String; m],
    }

    let mut counts = vec![0_usize; n];
    for t in &tt {
        if let Some(pos) = ss.iter().position(|s| s == t) {
            counts[pos] += 1;
        } else {
            let positions = ss
                .iter()
                .positions(|s| {
                    s.len() == t.len()
                        && s.chars()
                            .zip(t.chars())
                            .filter(|(ch1, ch2)| ch1 != ch2)
                            .count()
                            == 1
                })
                .collect_vec();
            if positions.len() == 1 {
                counts[positions[0]] += 1;
            }
        }
    }

    println!("{}", counts.iter().join("\n"));
}
