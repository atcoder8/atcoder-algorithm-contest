use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [u32; n],
        bbb: [[Usize1]; q],
    }

    let ia = enumerate(aa)
        .sorted_unstable_by_key(|&(_, a)| a)
        .collect_vec();

    let solve = |bb: &[usize]| {
        ia.iter()
            .find_map(|&(i, a)| if !bb.contains(&i) { Some(a) } else { None })
            .unwrap()
    };

    let output = bbb.iter().map(|bb| solve(bb)).join("\n");
    println!("{output}");
}
