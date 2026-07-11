use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        rc: [(Usize1, Usize1); m],
    }

    let mut horizontal = vec![false; n];
    let mut vertical = vec![false; n];
    let ans = rc
        .iter()
        .rev()
        .filter(|&&(r, c)| {
            let res = !horizontal[r] && !vertical[c];
            horizontal[r] = true;
            vertical[c] = true;
            res
        })
        .count();
    println!("{ans}");
}
