use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (x, y, z): (usize, usize, usize),
    }

    let (s1, s2, s3) = solve(x, y, z);
    let output = [s1, s2, s3]
        .map(|s| format!("{} {}", s.len(), s.iter().join(" ")))
        .join("\n");
    println!("{output}");
}

fn solve(x: usize, y: usize, z: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    if x == y {
        return (vec![0; x], vec![0; z], vec![0; z]);
    }

    let mut s1 = vec![0; y];
    s1[x..].fill(1);
    let s2 = vec![0; z];
    let mut s3 = vec![0; y + z];
    s3[x..y].fill(1);
    (s1, s2, s3)
}
