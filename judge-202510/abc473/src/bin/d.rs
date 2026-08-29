use itertools::Itertools;
use proconio::input;
use smallvec::SmallVec;

fn main() {
    input! {
        (n, k): (usize, u64),
    }

    let mut sequences = vec![];
    let mut stack: Vec<(SmallVec<[u64; 10]>, u64)> = vec![(SmallVec::new(), 0)];
    while let Some((mut seq, sum)) = stack.pop() {
        if seq.len() == n - 1 {
            seq.push(k - sum);
            seq.reverse();
            sequences.push(seq);

            continue;
        }

        stack.extend((0..=(k - sum) / (n - seq.len()) as u64).map(|a| {
            let mut next_seq = seq.clone();
            next_seq.push(a);
            (next_seq, sum + (n - seq.len()) as u64 * a)
        }));
    }

    sequences.sort_unstable();

    let output = sequences.iter().map(|seq| seq.iter().join(" ")).join("\n");
    println!("{output}");
}
