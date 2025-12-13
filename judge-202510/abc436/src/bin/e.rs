use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
    }

    let mut cycles = vec![];
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut curr = start;
        let mut cycle = vec![];
        while !visited[curr] {
            cycle.push(curr);
            visited[curr] = true;
            curr = pp[curr];
        }
        cycles.push(cycle);
    }

    let ans = cycles
        .iter()
        .map(|cycle| cycle.len() * (cycle.len() - 1) / 2)
        .sum::<usize>();
    println!("{ans}");
}
